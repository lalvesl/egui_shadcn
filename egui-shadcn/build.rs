use std::{fs, path::PathBuf};

const FONT_URL: &str =
    "https://github.com/google/material-design-icons/raw/master/font/MaterialIcons-Regular.ttf";

fn main() {
    // declare custom cfg so rustc doesn't warn about unknown cfg
    println!("cargo::rustc-check-cfg=cfg(has_material_icons)");

    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let assets   = manifest.join("assets");
    let ttf_path = assets.join("MaterialIcons-Regular.ttf");

    println!("cargo:rerun-if-changed=assets/MaterialIcons-Regular.ttf");
    println!("cargo:rerun-if-changed=build.rs");

    if !ttf_path.exists() {
        fs::create_dir_all(&assets).expect("create assets dir");
        eprintln!("[egui-shadcn] downloading Material Icons font…");

        match ureq::get(FONT_URL).call() {
            Ok(resp) => {
                let mut bytes = Vec::new();
                use std::io::Read;
                resp.into_reader()
                    .read_to_end(&mut bytes)
                    .expect("read font bytes");
                fs::write(&ttf_path, &bytes).expect("write font file");
                eprintln!("[egui-shadcn] font saved → {}", ttf_path.display());
            }
            Err(e) => {
                eprintln!("[egui-shadcn] WARNING: could not download font: {e}");
                eprintln!("[egui-shadcn] Material Icons will not render on native builds.");
            }
        }
    }

    if ttf_path.exists() {
        println!("cargo:rustc-cfg=has_material_icons");
    }
}
