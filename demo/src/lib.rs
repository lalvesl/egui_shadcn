pub mod app;
pub mod fonts;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    use app::DemoApp;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("egui_canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    let ctx = cc.egui_ctx.clone();
                    fetch_font(
                        "https://github.com/google/material-design-icons/raw/master/font/MaterialIcons-Regular.ttf",
                        move |bytes| egui_shadcn::register_font_bytes(&ctx, bytes),
                    );

                    let ctx = cc.egui_ctx.clone();
                    fetch_font(
                        "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/JetBrainsMono/Ligatures/Regular/JetBrainsMonoNerdFont-Regular.ttf",
                        move |bytes| crate::fonts::register_nerd_font_bytes(&ctx, bytes),
                    );

                    let ctx = cc.egui_ctx.clone();
                    fetch_font(
                        "https://github.com/googlefonts/roboto/raw/main/src/hinted/Roboto-Regular.ttf",
                        move |bytes| crate::fonts::register_roboto_bytes(&ctx, bytes),
                    );

                    Ok(Box::new(DemoApp::new(cc)))
                }),
            )
            .await
            .expect("failed to start eframe");
    });

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn fetch_font(url: &'static str, on_done: impl FnOnce(Vec<u8>) + Send + 'static) {
    ehttp::fetch(ehttp::Request::get(url), move |result| {
        if let Ok(resp) = result {
            if resp.ok {
                on_done(resp.bytes);
            }
        }
    });
}
