mod window;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    // Prettier errors
    console_error_panic_hook::set_once();

    // Make windows
    let document = web_sys::window().unwrap().document().unwrap();
    document
        .query_selector("body .title")
        .unwrap()
        .expect("Couldnt find `.title` element in `body`")
        .set_text_content(Some("stellarst0rm.github.io"));

    window::create_window(&document, "Terminal", ">|");
    window::create_window(&document, "Monitor", "%");
    window::create_window(&document, "Email", "@");

    Ok(())
}
