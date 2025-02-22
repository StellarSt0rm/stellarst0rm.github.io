mod closures;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Document, Element, MouseEvent};

pub struct Desktop {
    document: Document,
    window_container: Element,
    icon_container: Element,
}

impl Desktop {
    pub fn new(window_container: Element, icon_container: Element) -> Desktop {
        let document = web_sys::window().unwrap().document().unwrap();

        Desktop {
            document,
            window_container,
            icon_container,
        }
    }

    pub fn new_window(&mut self, window_data: &super::data::WindowData) {
        let window_id = format!(
            "{}{}",
            window_data.id(),
            self.window_container.children().length()
        );

        // Make window
        let window_element = self.document.create_element("div").unwrap();

        window_element.set_class_name("window");
        window_element.set_id(&window_id);
        window_element.set_inner_html(&window_template(
            &window_data.name().to_string(),
            &window_id,
            &window_data.html_content().to_string(),
        ));

        self.window_container.append_child(&window_element).unwrap();

        // Make launch bar icon
        let icon_element = self.document.create_element("div").unwrap();

        icon_element.set_class_name("icon");
        icon_element.set_id(&format!("{}-icon", window_id));
        icon_element.set_attribute("window", &window_id).unwrap();
        icon_element.set_inner_html(&icon_template(
            &window_data.name().to_string(),
            &window_data.icon_url().to_string(),
        ));

        self.icon_container.append_child(&icon_element).unwrap();

        // Add event closures to window, and pass to callback
        self.set_event_closures(window_element.clone());
        window_data.callback(window_element, &self.document);
    }

    fn set_event_closures(&self, window_element: Element) {
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

fn window_template(name: &String, window_id: &String, content: &String) -> String {
    let mut template = include_str!("data/window.template").to_string();

    template = template.replace("{{ name }}", name);
    template.replace(
        "{{ content }}",
        &content.replace("{{ window_id }}", window_id),
    )
}

fn icon_template(name: &String, icon_url: &String) -> String {
    let mut template = include_str!("data/icon.template").to_string();

    template = template.replace("{{ name }}", name);
    template.replace("{{ icon_url }}", icon_url)
}
