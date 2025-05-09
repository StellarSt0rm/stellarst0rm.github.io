use wasm_bindgen::prelude::JsCast;
use web_sys::{DomRect, Element, HtmlElement, MouseEvent};

// // // // //
// Closures //
// // // // //
pub fn mousedown(window_element: Element, event: MouseEvent) {
    // Make sure the function only runs on left clicks
    if event.button() != 0 {
        return;
    }

    // Store variables
    let topbar = window_element.query_selector(".topbar").unwrap().unwrap();

    let offset_x = event.client_x() as f64 - topbar.get_bounding_client_rect().left();
    let offset_y = event.client_y() as f64 - topbar.get_bounding_client_rect().top();

    window_element
        .set_attribute("offset", &format!("{offset_x}:{offset_y}"))
        .unwrap();
    window_element.set_attribute("mousedown", "true").unwrap();

    // Focus window
    super::Desktop::focus_window(&window_element);
}

pub fn mousemove(window_element: Element, event: MouseEvent) {
    let mousedown: bool = window_element
        .get_attribute("mousedown")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);

    let style = window_element
        .clone()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .style();

    if !mousedown {
        return;
    }

    // Parse offset
    let offset: Vec<f64> = window_element
        .get_attribute("offset")
        .unwrap_or("0:0".to_string())
        .split(":")
        .map(|offset| {
            offset
                .parse::<f64>()
                .expect("[mousemove] Failed to parse `offset` attribute")
        })
        .collect();

    if offset.len() != 2 {
        panic!("[mousemove] Wrong number of elements in `offset` attribute");
    }

    // Get new window position
    let container_rect = window_element
        .parent_element()
        .unwrap()
        .get_bounding_client_rect();
    let window_rect = window_element.get_bounding_client_rect();

    let old_left: f64 = style
        .get_property_value("left")
        .unwrap()
        .replace("px", "")
        .parse()
        .unwrap_or(0.); // If it's invalid, defaults to 0

    let left = event.client_x() as f64 - offset[0] - container_rect.left();
    let top = event.client_y() as f64 - offset[1] - container_rect.top();

    let (left, top) = apply_constraints(left, top, old_left, container_rect, window_rect);

    // Apply new position
    style.set_property("left", &format!("{left}px")).unwrap();
    style.set_property("top", &format!("{top}px")).unwrap();
}

fn apply_constraints(
    left: f64,
    top: f64,
    old_left: f64,

    container_rect: DomRect,
    window_rect: DomRect,
) -> (f64, f64) {
    // Status variables
    let mut window_limits = window_rect.width() / 3.;
    let limits_margin = 30.;

    // Check if window is inside the container
    if old_left >= 0. && (old_left + window_rect.width()) <= container_rect.width() {
        window_limits = window_rect.width();

        if (left + window_rect.width() - limits_margin) >= container_rect.width()
            || (left + limits_margin) <= 0.
        {
            window_limits = window_rect.width() / 3.;
        }
    }

    // Apply constraints
    let min_left = f64::min(left, container_rect.width() - window_limits);
    let min_top = f64::min(top, container_rect.height() - window_rect.height());

    let new_left = f64::max(-window_rect.width() + window_limits, min_left);
    let new_top = f64::max(0., min_top);

    (new_left, new_top)
}

// // // // //
// Actions  //
// // // // //
pub fn icon_pressed(window_element: Element, icon: Element) {
    super::Desktop::focus_window(&window_element);
}
