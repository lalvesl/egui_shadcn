use std::{fs, io::Read, path::PathBuf};

const NERD_FONT_URL: &str = "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/JetBrainsMono/Ligatures/Regular/JetBrainsMonoNerdFont-Regular.ttf";

const FALLBACK_FONT_URL: &str = "https://github.com/googlefonts/roboto/raw/main/src/hinted/Roboto-Regular.ttf";

const MATERIAL_ICONS_URL: &str = "https://github.com/google/material-design-icons/raw/master/font/MaterialIcons-Regular.ttf";

fn download_bytes(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let resp = ureq::get(url).call()?;
    let mut bytes = Vec::new();
    resp.into_body().into_reader().read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn fetch_font(out_path: &PathBuf, url: &str, label: &str) -> bool {
    if out_path.exists() {
        return true;
    }
    eprintln!("[demo] downloading {label}…");
    match download_bytes(url) {
        Ok(bytes) => {
            fs::write(out_path, &bytes).expect("write font");
            eprintln!("[demo] saved → {}", out_path.display());
            true
        }
        Err(e) => {
            eprintln!("[demo] WARNING: could not download {label}: {e}");
            false
        }
    }
}

// skrifa's autohint_shaping reads GPOS/GSUB tables. The minimal stub tables in
// MaterialIcons trigger ReadError::OffsetOutOfBounds on 32-bit wasm32. Rename
// those tags to unknown values so skrifa silently skips them. Glyph outlines
// are unaffected — GPOS/GSUB only carry kerning/substitution rules.
fn strip_gpos_gsub(ttf: &[u8]) -> Vec<u8> {
    let mut out = ttf.to_vec();
    if out.len() < 12 {
        return out;
    }
    let num_tables = u16::from_be_bytes([out[4], out[5]]) as usize;
    for i in 0..num_tables {
        let base = 12 + i * 16;
        if base + 4 > out.len() {
            break;
        }
        let tag = &out[base..base + 4];
        if tag == b"GPOS" || tag == b"GSUB" {
            out[base] = b'X';
        }
    }
    out
}

fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_nerd_font)");
    println!("cargo::rustc-check-cfg=cfg(has_fallback_font)");
    println!("cargo::rustc-check-cfg=cfg(has_wasm_icons)");
    println!("cargo:rerun-if-changed=build.rs");

    let target_arch =
        std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    if target_arch == "wasm32" {
        let manifest_dir =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let wasm_assets = manifest_dir.join("../target/wasm_assets");
        fs::create_dir_all(&wasm_assets).expect("create target/wasm_assets");

        let raw_path = wasm_assets.join("MaterialIcons-Regular.ttf.raw");
        let out_path = wasm_assets.join("MaterialIcons-Regular.ttf");

        if !out_path.exists() {
            if !raw_path.exists() {
                eprintln!("[demo] downloading Material Icons…");
                match download_bytes(MATERIAL_ICONS_URL) {
                    Ok(bytes) => {
                        fs::write(&raw_path, &bytes).expect("write raw font");
                    }
                    Err(e) => {
                        eprintln!(
                            "[demo] WARNING: could not download Material Icons: {e}"
                        );
                        return;
                    }
                }
            }
            let raw = fs::read(&raw_path).expect("read raw font");
            let stripped = strip_gpos_gsub(&raw);
            fs::write(&out_path, &stripped).expect("write stripped font");
            eprintln!("[demo] saved stripped font → {}", out_path.display());
        }

        // Emit env var so the Rust code can include_bytes! the font at compile time.
        // This embeds the font in the WASM binary — no browser caching, no ehttp race.
        println!("cargo:rerun-if-changed={}", out_path.display());
        println!("cargo:rustc-env=WASM_MATERIAL_ICONS={}", out_path.display());
        println!("cargo:rustc-cfg=has_wasm_icons");
        return;
    }

    // Native: download to OUT_DIR and embed via include_bytes!
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    if fetch_font(
        &out_dir.join("nerd_font.ttf"),
        NERD_FONT_URL,
        "JetBrainsMono Nerd Font",
    ) {
        println!("cargo:rustc-cfg=has_nerd_font");
    }

    if fetch_font(
        &out_dir.join("fallback_font.ttf"),
        FALLBACK_FONT_URL,
        "Roboto",
    ) {
        println!("cargo:rustc-cfg=has_fallback_font");
    }
}
