use std::{fs, io::Read, path::PathBuf};

const NERD_FONT_URL: &str =
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/JetBrainsMono/Ligatures/Regular/JetBrainsMonoNerdFont-Regular.ttf";

const FALLBACK_FONT_URL: &str =
    "https://github.com/googlefonts/roboto/raw/main/src/hinted/Roboto-Regular.ttf";

const MATERIAL_ICONS_URL: &str =
    "https://github.com/google/material-design-icons/raw/master/font/MaterialIcons-Regular.ttf";

fn download_bytes(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let resp = ureq::get(url).call()?;
    let mut bytes = Vec::new();
    resp.into_reader().read_to_end(&mut bytes)?;
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

fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_nerd_font)");
    println!("cargo::rustc-check-cfg=cfg(has_fallback_font)");
    println!("cargo:rerun-if-changed=build.rs");

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    if target_arch == "wasm32" {
        // For web: download fonts to target/wasm_assets/ so Trunk serves them as
        // static assets. The WASM binary fetches them at runtime via ehttp.
        let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let wasm_assets = manifest_dir.join("../target/wasm_assets");
        fs::create_dir_all(&wasm_assets).expect("create target/wasm_assets");
        fetch_font(
            &wasm_assets.join("MaterialIcons-Regular.ttf"),
            MATERIAL_ICONS_URL,
            "Material Icons",
        );
        fetch_font(
            &wasm_assets.join("JetBrainsMonoNerdFont-Regular.ttf"),
            NERD_FONT_URL,
            "JetBrains Mono Nerd Font",
        );
        fetch_font(
            &wasm_assets.join("Roboto-Regular.ttf"),
            FALLBACK_FONT_URL,
            "Roboto",
        );
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
