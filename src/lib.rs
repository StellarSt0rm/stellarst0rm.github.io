mod desktop;

use console_error_panic_hook;
use desktop::Desktop;
use wasm_bindgen::prelude::*;
use web_sys;

// Logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
// ---

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    // Better error logging!
    console_error_panic_hook::set_once();

    // Clear stuff
    let document = web_sys::window().unwrap().document().unwrap();

    let loading_div = document
        .get_element_by_id("loading")
        .expect("Couldnt find 'loading' div");
    loading_div.remove();

    // Load windows!
    let window_container = document.get_element_by_id("window_container").unwrap();
    let data = include_str!("data/windows.toml")
        .parse::<toml::Table>()
        .unwrap();
    let mut desktop = Desktop::new(window_container);

    for id in data.keys() {
        let window = data[id].clone();

        let name = window["name"].as_str().unwrap().to_string();
        let content = window["content"].as_str().unwrap().to_string();
        let icon_url = window["icon_url"].as_str().unwrap().to_string();

        desktop.new_window(name, content, icon_url);
    }

    Ok(())
}
