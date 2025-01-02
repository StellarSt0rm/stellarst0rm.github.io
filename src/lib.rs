use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    // Better error logging!
    console_error_panic_hook::set_once();

    // App
    let document = web_sys::window().unwrap().document().unwrap();

    let lang_span = document.get_element_by_id("lang").unwrap();
    lang_span.set_text_content(Some("Rust"));

    Ok(())
}
