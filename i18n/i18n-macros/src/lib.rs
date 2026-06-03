//! `i18n-macros` — the compile-time half of the system.
//!
//! * `#[traductions]` parses the translation DSL, computes the salted `APP_ID`,
//!   regenerates a payload-free enum, implements `Translate`, and (per enabled
//!   language feature) embeds one binary catalog as a `&'static [u8]` const,
//!   registering it with the runtime via a `linkme` distributed slice.
//! * `t!` expands a lookup at a call site into a `Cow<'static, str>`, suitable
//!   for handing straight to `ui.label(..)` in egui.

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, LitStr, Token,
};

// ---------------------------------------------------------------------------
// APP_ID hashing — FNV-1a (32-bit) over salt + enum name, folded to u16.
// Done here, at expansion time, so the result is emitted as a `u16` literal.
// ---------------------------------------------------------------------------

/// Constant salt baked into the macro. Changing it reshuffles every id, so it is
/// fixed for the format version. (You could expose it via an env var if you ever
/// need per-build namespacing.)
const APP_ID_SALT: &str = "i18n::v1::application::";

fn fnv1a_16(name: &str) -> u16 {
    let mut hash: u32 = 0x811c_9dc5;
    for b in APP_ID_SALT.bytes().chain(name.bytes()) {
        hash ^= b as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    // Fold the 32-bit hash into 16 bits (xor-fold) to keep the full entropy.
    ((hash >> 16) ^ (hash & 0xFFFF)) as u16
}

// Map a DSL language identifier to (feature, discriminant). Kept inline at the
// call sites below; this list is the single source of truth for the four langs.

// ---------------------------------------------------------------------------
// DSL parsing for #[traductions]
//
//   #[i18n::traductions]
//   enum Calendar {
//       January([En("January"), PtBr("Janeiro")]),
//       Month([En("The month {month}"), PtBr("O mês {month}")]),
//       Apples(plural {
//           one:   [En("{n} apple"),  PtBr("{n} maçã")],
//           other: [En("{n} apples"), PtBr("{n} maçãs")],
//       }),
//   }
// ---------------------------------------------------------------------------

/// One `Lang("literal")` pair inside a form array.
struct LangValue {
    lang: Ident,
    value: LitStr,
}

impl Parse for LangValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lang: Ident = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let value: LitStr = content.parse()?;
        Ok(LangValue { lang, value })
    }
}

/// `[En("..."), PtBr("...")]`
struct FormArray {
    values: Vec<LangValue>,
}

impl Parse for FormArray {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::bracketed!(content in input);
        let items: Punctuated<LangValue, Token![,]> =
            content.parse_terminated(LangValue::parse, Token![,])?;
        Ok(FormArray {
            values: items.into_iter().collect(),
        })
    }
}

/// The body of one enum variant in the DSL: either a single form array, or a
/// `plural { category: [..], .. }` block.
enum VariantBody {
    Plain(FormArray),
    Plural(Vec<(Ident, FormArray)>),
}

/// A parsed DSL variant.
struct DslVariant {
    name: Ident,
    body: VariantBody,
}

impl Parse for DslVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        syn::parenthesized!(content in input);

        // Is it the `plural { .. }` form?
        if content.peek(Ident)
            && content
                .fork()
                .parse::<Ident>()
                .map(|i| i == "plural")
                .unwrap_or(false)
        {
            let _plural: Ident = content.parse()?;
            let inner;
            syn::braced!(inner in content);
            let mut forms = Vec::new();
            while !inner.is_empty() {
                let cat: Ident = inner.parse()?;
                inner.parse::<Token![:]>()?;
                let arr: FormArray = inner.parse()?;
                forms.push((cat, arr));
                if inner.peek(Token![,]) {
                    inner.parse::<Token![,]>()?;
                }
            }
            Ok(DslVariant {
                name,
                body: VariantBody::Plural(forms),
            })
        } else {
            let arr: FormArray = content.parse()?;
            Ok(DslVariant {
                name,
                body: VariantBody::Plain(arr),
            })
        }
    }
}

/// The whole DSL enum.
struct DslEnum {
    vis: syn::Visibility,
    name: Ident,
    variants: Vec<DslVariant>,
}

impl Parse for DslEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Tolerate (and discard) any leftover attributes on the item.
        let _attrs = input.call(syn::Attribute::parse_outer)?;
        let vis: syn::Visibility = input.parse()?;
        input.parse::<Token![enum]>()?;
        let name: Ident = input.parse()?;
        let content;
        syn::braced!(content in input);
        let variants: Punctuated<DslVariant, Token![,]> =
            content.parse_terminated(DslVariant::parse, Token![,])?;
        Ok(DslEnum {
            vis,
            name,
            variants: variants.into_iter().collect(),
        })
    }
}

/// `traductions! { ... }` entry point.
///
/// This is a *function-like* macro, not an attribute, because the array DSL
/// (`January([En("January"), PtBr("Janeiro")])`) is not a grammatically valid
/// enum — `En("January")` is not a type — and attribute macros require their
/// item to parse. A function-like macro takes arbitrary token trees, so the
/// exact DSL is preserved.
#[proc_macro]
pub fn traductions(item: TokenStream) -> TokenStream {
    let dsl = parse_macro_input!(item as DslEnum);
    let DslEnum {
        vis,
        name,
        variants,
    } = dsl;

    let app_id = fnv1a_16(&name.to_string());

    // 1. The regenerated, payload-free enum.
    let variant_idents: Vec<&Ident> = variants.iter().map(|v| &v.name).collect();
    let enum_def = quote! {
        #[repr(u8)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #vis enum #name {
            #(#variant_idents,)*
        }

        impl ::i18n::Translate for #name {
            const APP_ID: u16 = #app_id;
            #[inline]
            fn variant_index(self) -> u8 { self as u8 }
        }
    };

    // 2. For each language, build the catalog bytes *now* and emit a const blob
    //    guarded by that language's feature, plus a linkme registration.
    let mut lang_blobs = TokenStream2::new();

    for (lang_variant, feature, lang_disc) in [
        ("En", "lang-en", 0u8),
        ("EnUs", "lang-en-us", 1),
        ("Pt", "lang-pt", 2),
        ("PtBr", "lang-pt-br", 3),
    ] {
        let lang_id = format_ident!("{lang_variant}");

        // Gather this language's entries by walking the DSL.
        let mut entries: Vec<i18n_format::EntrySpec> = Vec::new();
        for (idx, var) in variants.iter().enumerate() {
            let forms = collect_forms_for_lang(var, lang_variant);
            if forms.is_empty() {
                continue; // this variant has no string for this language
            }
            entries.push(i18n_format::EntrySpec {
                app_id,
                variant: idx as u8,
                forms,
            });
        }

        if entries.is_empty() {
            continue;
        }

        let lang_enum = match lang_disc {
            0 => i18n_format::Languages::En,
            1 => i18n_format::Languages::EnUs,
            2 => i18n_format::Languages::Pt,
            _ => i18n_format::Languages::PtBr,
        };
        let bytes = i18n_format::encode_catalog(lang_enum, entries);
        let byte_lit = proc_macro2::Literal::byte_string(&bytes);

        let const_ident = format_ident!(
            "__I18N_{}_{}",
            name.to_string().to_uppercase(),
            lang_variant.to_uppercase()
        );
        let slice_ident = format_ident!("__I18N_CATALOGS_{}", lang_variant.to_uppercase());

        lang_blobs.extend(quote! {
            // linkme has no wasm backend, so the registration only exists on
            // native targets; on wasm the registry slices are empty and lookups
            // fall through to the async `Source`.
            #[cfg(all(feature = #feature, not(target_arch = "wasm32")))]
            const _: () = {
                static #const_ident: &[u8] = #byte_lit;
                #[::i18n::linkme::distributed_slice(::i18n::registry::#slice_ident)]
                #[linkme(crate = ::i18n::linkme)]
                static REGISTER: ::i18n::format::Catalog<'static> =
                    ::i18n::format::Catalog::new_unchecked(#const_ident);
                // Touch the language discriminant so a typo here is a compile error.
                let _ = ::i18n::Languages::#lang_id;
            };
        });
    }

    let expanded = quote! {
        #enum_def
        #lang_blobs
    };
    expanded.into()
}

/// Helper: turn a DSL variant into the encoder's `FormSpec`s for one language.
fn collect_forms_for_lang(var: &DslVariant, lang_variant: &str) -> Vec<i18n_format::FormSpec> {
    use i18n_format::{FormSpec, Plural};
    let cat_of = |c: &str| -> Plural {
        match c {
            "zero" => Plural::Zero,
            "one" => Plural::One,
            "two" => Plural::Two,
            "few" => Plural::Few,
            "many" => Plural::Many,
            _ => Plural::Other,
        }
    };
    match &var.body {
        VariantBody::Plain(arr) => arr
            .values
            .iter()
            .filter(|lv| lv.lang == lang_variant)
            .map(|lv| FormSpec {
                category: Plural::Other,
                template: lv.value.value(),
            })
            .collect(),
        VariantBody::Plural(forms) => {
            let mut out = Vec::new();
            for (cat, arr) in forms {
                for lv in &arr.values {
                    if lv.lang == lang_variant {
                        out.push(FormSpec {
                            category: cat_of(&cat.to_string()),
                            template: lv.value.value(),
                        });
                    }
                }
            }
            out
        }
    }
}

// ---------------------------------------------------------------------------
// t! macro
//
//   t!(Calendar::January)                      -> Cow<'static, str>
//   t!(Calendar::Month, month = m)             -> formatted
//   t!(Calendar::Apples, count = n)            -> plural-selected
//   t!(Calendar::Apples, count = n, fruit = f) -> plural + args
// ---------------------------------------------------------------------------

struct TInvocation {
    item: Expr,          // e.g. `Calendar::January`
    count: Option<Expr>, // from `count = ...`
    args: Vec<(Ident, Expr)>,
}

impl Parse for TInvocation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item: Expr = input.parse()?;
        let mut count = None;
        let mut args = Vec::new();
        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            let name: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let val: Expr = input.parse()?;
            if name == "count" {
                count = Some(val);
            } else {
                args.push((name, val));
            }
        }
        Ok(TInvocation { item, count, args })
    }
}

#[proc_macro]
pub fn t(input: TokenStream) -> TokenStream {
    let TInvocation { item, count, args } = parse_macro_input!(input as TInvocation);

    let arg_tokens = args.iter().map(|(name, val)| {
        let lit = LitStr::new(&name.to_string(), Span::call_site());
        quote! {
            ::i18n::format::Arg::new(#lit, ::i18n::format::ArgValue::from(#val))
        }
    });

    let count_tokens = match count {
        Some(c) => quote! { ::core::option::Option::Some((#c) as i64) },
        None => quote! { ::core::option::Option::None },
    };

    let expanded = quote! {{
        // `item` must implement Translate; this also enforces it at the call site.
        let __item = #item;
        let __args = [ #(#arg_tokens),* ];
        ::i18n::translate(
            <_ as ::i18n::Translate>::translation_id(__item),
            ::i18n::Translate::variant_index(__item),
            #count_tokens,
            &__args,
        )
    }};
    expanded.into()
}
