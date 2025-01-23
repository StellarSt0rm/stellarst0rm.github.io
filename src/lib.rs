mod data;
mod desktop;

use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::{self};

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    // Better error logging!
    console_error_panic_hook::set_once();

    // Load app
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document
        .get_element_by_id("window_container")
        .expect("[start] Couldnt find `window_container` element");
    let mut desktop = desktop::Desktop::new(container);

    for window in data::WINDOWS.iter() {
        desktop.new_window(window);
    }

    // Other
    let version_span = document.get_element_by_id("version").unwrap();
    version_span.set_text_content(Some(env!("CARGO_PKG_VERSION")));

    // Show app
    let loading_div = document
        .get_element_by_id("loading")
        .expect("[start] Couldnt find `loading` element");
    loading_div.set_attribute("hidden", "true").unwrap();

    let bottom_div = document
        .get_element_by_id("bottom")
        .expect("[start] Couldnt find `bottom` element");
    bottom_div.set_attribute("hidden", "false").unwrap();

    Ok(())
}
