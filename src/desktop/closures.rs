use wasm_bindgen::prelude::JsCast;
use web_sys::{DomRect, Element, HtmlElement, MouseEvent};

pub fn mosuedown(window_element: Element, event: MouseEvent) {
    // Store variables
    let topbar = window_element.query_selector(".topbar").unwrap().unwrap();

    let offset_x = event.client_x() as f64 - topbar.get_bounding_client_rect().left();
    let offset_y = event.client_y() as f64 - topbar.get_bounding_client_rect().top();

    window_element
        .set_attribute("offset", &format!("{offset_x}:{offset_y}"))
        .unwrap();
    window_element.set_attribute("mousedown", "true").unwrap();

    // Get the container, we *know* the window has a parent, so we just unwrap.
    let windows = window_element.parent_element().unwrap().children();

    for id in 0..windows.length() {
        // Since we're going through items based on the collection length,
        // we dont need to ensure the item exists.
        let window = windows.item(id).unwrap();

        web_sys::console::log_1(&window.into());
    }
}

pub fn mousemove(window_element: Element, event: MouseEvent) {
    let mousedown: bool = window_element
        .get_attribute("mousedown")
        .unwrap_or("false".to_string())
        .parse()
        .expect("[mousemove_closure] Failed to parse `mousedown` status variable");

    if !mousedown {
        return;
    }

    // Parse offset
    let offset = window_element
        .get_attribute("offset")
        .unwrap_or("0:0".to_string())
        .split(":")
        .map(|offset| {
            offset
                .parse::<f64>()
                .expect("[mousemove_closure] Failed to parse `offset` status variable")
        })
        .collect::<Vec<f64>>();

    if offset.len() != 2 {
        panic!("[mousemove_closure] Wrong number of elements in `offset` status variable");
    }

    // Get new window position
    let container_rect = window_element
        .parent_element()
        .unwrap()
        .get_bounding_client_rect();
    let window_rect = window_element.get_bounding_client_rect();

    let left = event.client_x() as f64 - offset[0] - container_rect.left();
    let top = event.client_y() as f64 - offset[1] - container_rect.top();

    let (left, top) = apply_constraints(left, top, container_rect, window_rect);

    // Apply new position
    let style = window_element
        .clone()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .style();

    style.set_property("left", &format!("{left}px")).unwrap();
    style.set_property("top", &format!("{top}px")).unwrap();
}

fn apply_constraints(
    left: f64,
    top: f64,
    container_rect: DomRect,
    window_rect: DomRect,
) -> (f64, f64) {
    (left, top)
}
