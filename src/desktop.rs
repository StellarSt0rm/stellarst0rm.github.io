mod closures;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Document, Element, MouseEvent};

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

    pub fn new_window(&mut self, window_data: &super::data::WindowData) {
        let window_element = self.document.create_element("div").unwrap();

        window_element.set_class_name("window");
        window_element.set_inner_html(&window_template(
            &window_data.name().to_string(),
            &window_data.html_content().to_string(),
        ));
        self.container.append_child(&window_element).unwrap();

        self.make_window_draggable(window_element.clone());

        // Pass the window to the callback
        window_data.callback(window_element, &self.document);
    }

    fn make_window_draggable(&self, window_element: Element) {
        let topbar = window_element
            .query_selector(".topbar")
            .unwrap() // This fails when there's a syntax error
            .expect("[drag_initalizer] Couldnt find `topbar` element");

        // Closure
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            let event_type = event.type_();

            match event_type.as_str() {
                "mousedown" => closures::mousedown(window_element.clone(), event),
                "mouseup" => window_element.set_attribute("mousedown", "false").unwrap(),
                "mousemove" => closures::mousemove(window_element.clone(), event),

                _ => panic!("Cant to handle `{event_type}`"),
            }
        });

        // Add callbacks
        topbar
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        self.document
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .unwrap();
        self.document
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();

        // Toss it into the void!
        closure.forget();
    }
}

fn window_template(name: &String, content: &String) -> String {
    let mut template = include_str!("data/window.template").to_string();

    template = template.replace("{{ name }}", name);
    template.replace("{{ content }}", content)
}
