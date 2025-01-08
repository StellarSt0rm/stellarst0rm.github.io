use wasm_bindgen::prelude::*;
use web_sys::Element;

pub fn make_draggable(window: &Element) {
    let id = window.id();
    let topbar = window.query_selector(".topbar").unwrap().unwrap(); // Yes, TWO unwraps in a row. 😭

    let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        web_sys::console::log_1(&format!("Window: {}", id).into());
    });

    topbar
        .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget(); // Memory leaks, nyom nyom.
}
