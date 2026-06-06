// Catalog generator: `demo-native --gen-i18n <dir>` (or GEN_I18N_DIR=<dir>)
// writes one `{bcp47}.cat` bundle per language into <dir> and exits. Build this
// with every language feature on so all four are exported:
//   cargo run -p demo --bin demo-native --features lang-en,lang-pt -- --gen-i18n <dir>
// The demo binary statically reaches every section (hence every `traductions!`
// enum), so linkme collects all catalogs — embedded + component — into the
// per-language bundles. Run by Trunk before the wasm build.
#[cfg(not(target_arch = "wasm32"))]
fn gen_i18n_dir() -> Option<std::path::PathBuf> {
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "--gen-i18n" {
            return args.next().map(std::path::PathBuf::from);
        }
    }
    std::env::var("GEN_I18N_DIR")
        .ok()
        .map(std::path::PathBuf::from)
}

#[cfg(not(target_arch = "wasm32"))]
fn generate_catalogs(dir: &std::path::Path) {
    std::fs::create_dir_all(dir).expect("create i18n output dir");
    for lang in i18n::Languages::ALL {
        let bytes = i18n::export_catalogs(lang);
        let path = dir.join(format!("{}.cat", lang.bcp47()));
        std::fs::write(&path, &bytes).expect("write catalog bundle");
        eprintln!(
            "[gen-i18n] {} -> {} ({} bytes)",
            lang.bcp47(),
            path.display(),
            bytes.len()
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    if let Some(dir) = gen_i18n_dir() {
        generate_catalogs(&dir);
        return Ok(());
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("egui-shadcn demo")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "egui-shadcn",
        native_options,
        Box::new(|cc| Ok(Box::new(demo::app::DemoApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}
