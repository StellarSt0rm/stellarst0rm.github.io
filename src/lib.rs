mod desktop;
mod templates;

use console_error_panic_hook;
use desktop::Desktop;
use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    // Better error logging!
    console_error_panic_hook::set_once();

    // Load app
    let document = web_sys::window().unwrap().document().unwrap();
    let window_container = document.get_element_by_id("window_container").unwrap();
    let data = include_str!("data/windows.toml")
        .parse::<toml::Table>()
        .unwrap();
    let mut desktop = Desktop::new(window_container);

    for id_ in data.keys() {
        let window = data[id_].clone();

        let name = window["name"].as_str().unwrap().to_string();
        let content = window["content"].as_str().unwrap().to_string();
        let icon_url = window["icon_url"].as_str().unwrap().to_string();

        desktop.new_window(name, content, icon_url);
    }

    let version_span = document.get_element_by_id("version").unwrap();
    version_span.set_text_content(Some(env!("CARGO_PKG_VERSION")));

    // Show app
    let loading_div = document
        .get_element_by_id("loading")
        .expect("Couldnt find 'loading' div");
    loading_div.remove();

    Ok(())
}
