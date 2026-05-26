pub mod app;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    use app::DemoApp;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document  = web_sys::window().unwrap().document().unwrap();
        let canvas    = document.get_element_by_id("egui_canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

        let runner = eframe::WebRunner::new();
        runner
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(DemoApp::new(cc)))),
            )
            .await
            .expect("failed to start eframe");

        // After startup: fetch Material Icons font and register it
        fetch_and_register_font(runner.egui_ctx().unwrap()).await;
    });

    Ok(())
}

#[cfg(target_arch = "wasm32")]
async fn fetch_and_register_font(ctx: egui::Context) {
    let request = ehttp::Request::get("MaterialIcons-Regular.ttf");

    match ehttp::fetch_async(request).await {
        Ok(resp) if resp.ok => {
            let bytes = resp.bytes;
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "MaterialIcons".to_owned(),
                egui::FontData::from_owned(bytes),
            );
            fonts.families.insert(
                egui::FontFamily::Name("MaterialIcons".into()),
                vec!["MaterialIcons".to_owned()],
            );
            ctx.set_fonts(fonts);
        }
        _ => {
            web_sys::console::warn_1(&"Material Icons font could not be loaded on web.".into());
        }
    }
}
