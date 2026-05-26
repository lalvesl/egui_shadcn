pub mod app;

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
                    // Kick off async font fetch while the app is starting.
                    // register_font already registered a stub so no frame panics.
                    let ctx = cc.egui_ctx.clone();
                    ehttp::fetch(
                        ehttp::Request::get("MaterialIcons-Regular.ttf"),
                        move |result| {
                            if let Ok(resp) = result {
                                if resp.ok {
                                    egui_shadcn::register_font_bytes(&ctx, resp.bytes);
                                }
                            }
                        },
                    );

                    Ok(Box::new(DemoApp::new(cc)))
                }),
            )
            .await
            .expect("failed to start eframe");
    });

    Ok(())
}
