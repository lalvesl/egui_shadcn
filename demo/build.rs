use std::{fs, io::Read, path::PathBuf};

// JetBrainsMono Nerd Font — primary UI font with extra glyphs
const NERD_FONT_URL: &str =
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/JetBrainsMono/Ligatures/Regular/JetBrainsMonoNerdFont-Regular.ttf";

// Roboto Regular — metrically clean fallback (used when Nerd Font unavailable)
const FALLBACK_FONT_URL: &str =
    "https://github.com/googlefonts/roboto/raw/main/src/hinted/Roboto-Regular.ttf";

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
        "Roboto (Arial fallback)",
    ) {
        println!("cargo:rustc-cfg=has_fallback_font");
    }
}
