//use std::collections::HashMap;
use wasm_bindgen::prelude::{Closure, JsCast};
use web_sys::{Document, Element, MouseEvent};

pub struct Desktop {
    document: Document,
    container: Element,
    //closures: HashMap<u32, Vec<Closure<dyn FnMut(web_sys::MouseEvent)>>>,
}

impl Desktop {
    pub fn new(container: Element) -> Desktop {
        let document = web_sys::window().unwrap().document().unwrap();

        Desktop {
            document,
            container,
            //closures: HashMap::new(),
        }
    }

    pub fn new_window(&mut self, name: String, content: String) {
        let window_element = self.document.create_element("div").unwrap();
        let id = self.container.child_element_count() + 1;

        window_element.set_id(&id.to_string());
        window_element.set_class_name("window");

        window_element.set_inner_html(&window_template(&name, &content));
        self.container.append_child(&window_element).unwrap();

        // Make draggable, last action so we give ownership
        make_window_draggable(window_element, self.container.clone());
        //self.closures.insert(id, new_closures);
    }
}

fn window_template(name: &String, content: &String) -> String {
    let mut template = include_str!("data/window.template").to_string();

    template = template.replace("{{ name }}", name);
    template.replace("{{ content }}", content)
}

fn make_window_draggable(window_element: Element, container: Element)
/*-> Vec<Closure<dyn FnMut(MouseEvent)>>*/
{
    let topbar = window_element
        .query_selector(".topbar")
        .unwrap() // This fails when there's a syntax error
        .expect("Couldn't find 'topbar' element");

    // Callbacks
    let mousedown_window = window_element.clone();
    let mouseup_window = window_element.clone();

    let mousedown_closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        mousedown_window.set_attribute("mousedown", "true").unwrap();
    });
    let mouseup_closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        mouseup_window.set_attribute("mousedown", "false").unwrap();
    });

    let mousemove_closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        let mousedown: bool = window_element
            .get_attribute("mousedown")
            .unwrap_or("false".to_string())
            .parse()
            .expect("Failed to parse 'mousedown' status variable");

        if !mousedown {
            return;
        }

        web_sys::console::log_1(&"Drag!".into());
    });

    // Add callbacks
    topbar
        .add_event_listener_with_callback("mousedown", mousedown_closure.as_ref().unchecked_ref())
        .unwrap();
    topbar
        .add_event_listener_with_callback("mouseup", mouseup_closure.as_ref().unchecked_ref())
        .unwrap();

    topbar
        .add_event_listener_with_callback("mousemove", mousemove_closure.as_ref().unchecked_ref())
        .unwrap();

    // Toss 'em into the void
    mousedown_closure.forget();
    mouseup_closure.forget();
    mousemove_closure.forget();

    //vec![mousedown] // Trying to store closures generates a lot of errors
}
