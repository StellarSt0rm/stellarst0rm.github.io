use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    // Better error logging!
    console_error_panic_hook::set_once();

    // App
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let loading_div = document
        .get_element_by_id("loading")
        .expect("Couldnt find 'loading' div");
    loading_div.remove();

    Ok(())
}
