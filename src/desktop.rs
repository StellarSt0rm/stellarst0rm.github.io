use wasm_bindgen::prelude::{Closure, JsCast};
use web_sys::{Document, Element, HtmlElement, MouseEvent};

pub struct Desktop {
    document: Document,
    container: Element,
}

impl Desktop {
    pub fn new(container: Element) -> Desktop {
        let document = web_sys::window().unwrap().document().unwrap();

        Desktop {
            document,
            container,
        }
    }

    pub fn new_window(&mut self, name: String, content: String) {
        let window_element = self.document.create_element("div").unwrap();

        window_element.set_class_name("window");
        window_element.set_inner_html(&window_template(&name, &content));
        self.container.append_child(&window_element).unwrap();

        // Make draggable
        self.make_window_draggable(window_element);
    }

    fn make_window_draggable(&self, window_element: Element) {
        let topbar = window_element
            .query_selector(".topbar")
            .unwrap() // This fails when there's a syntax error
            .expect("Couldn't find 'topbar' element");

        // Callbacks
        let mousedown_window = window_element.clone();
        let mouseup_window = window_element.clone();

        let mousedown_closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            // Store variables
            let topbar = mousedown_window.query_selector(".topbar").unwrap().unwrap();

            let offset_x = event.client_x() as f64 - topbar.get_bounding_client_rect().left();
            let offset_y = event.client_y() as f64 - topbar.get_bounding_client_rect().top();

            mousedown_window
                .set_attribute("offset_x", &offset_x.to_string())
                .unwrap();
            mousedown_window
                .set_attribute("offset_y", &offset_y.to_string())
                .unwrap();
            mousedown_window.set_attribute("mousedown", "true").unwrap();

            // Get the container, we *know* the window has a parent, so we just unwrap.
            let windows = mousedown_window.parent_element().unwrap().children();

            for id in 0..windows.length() {
                // Since we're going through items based on the collection length,
                // we dont need to ensure the item exists.
                let window = windows.item(id).unwrap();

                web_sys::console::log_1(&window.into());
            }
        });
        let mouseup_closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
            mouseup_window.set_attribute("mousedown", "false").unwrap();
        });

        let mousemove_closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            let mousedown: bool = window_element
                .get_attribute("mousedown")
                .unwrap_or("false".to_string())
                .parse()
                .expect("Failed to parse 'mousedown' status variable");

            if !mousedown {
                return;
            }

            // Parse offset
            let offset_y: f64 = window_element
                .get_attribute("offset_y")
                .unwrap_or("0".to_string())
                .parse()
                .expect("Failed to parse 'offset_y' status variable");

            let offset_x: f64 = window_element
                .get_attribute("offset_x")
                .unwrap_or("0".to_string())
                .parse()
                .expect("Failed to parse 'offset_x' status variable");

            web_sys::console::log_1(&format!("{offset_x}, {offset_y}").into());

            // Get new window position
            let container_rect = window_element
                .parent_element()
                .unwrap()
                .get_bounding_client_rect();
            let window_rect = window_element.get_bounding_client_rect();

            let left = event.client_x() as f64 - offset_x - container_rect.left();
            let top = event.client_y() as f64 - offset_y - container_rect.top();

            // Math Time!

            // Apply new position
            let style = window_element
                .clone()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .style();

            style.set_property("left", &format!("{left}px")).unwrap();
            style.set_property("top", &format!("{top}px")).unwrap();

            web_sys::console::log_1(&format!("{offset_x}, {offset_y}").into());
        });

        // Add callbacks
        topbar
            .add_event_listener_with_callback(
                "mousedown",
                mousedown_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        self.document
            .add_event_listener_with_callback("mouseup", mouseup_closure.as_ref().unchecked_ref())
            .unwrap();

        self.document
            .add_event_listener_with_callback(
                "mousemove",
                mousemove_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        // Toss 'em into the void
        mousedown_closure.forget();
        mouseup_closure.forget();

        mousemove_closure.forget();
    }
}

fn window_template(name: &String, content: &String) -> String {
    let mut template = include_str!("data/window.template").to_string();

    template = template.replace("{{ name }}", name);
    template.replace("{{ content }}", content)
}
