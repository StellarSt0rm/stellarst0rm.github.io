mod drag;

use crate::templates;
use web_sys::{Document, Element};

pub struct Desktop {
    container: Element,
    document: Document,

    window_pool: Vec<Window>,
}

/// The 'layer' field goes from 0 to n, 0 being the topmost.
pub struct Window {
    pub layer: usize,
    pub id: usize,

    pub name: String,
    pub content: String,

    pub xy: (i32, i32),
}

impl Desktop {
    pub fn new(container: Element) -> Desktop {
        //! Create a new Desktop, a container must be given.
        let document = web_sys::window().unwrap().document().unwrap();

        Desktop {
            container,
            document,

            window_pool: Vec::new(),
        }
    }

    pub fn new_window(&mut self, name: String, content: String, icon_url: String) -> &Window {
        //! Create a new window, and add it to the container
        let window = Window {
            layer: 0, // Layer set to 0 for new windows.
            id: self.window_pool.len() + 1,

            name,
            content,

            xy: (0, 0),
        };

        // Create window in DOM
        let window_element = self.document.create_element("div").unwrap();
        let html = templates::window(&window.name, &window.content);

        window_element.set_id(&window.id.to_string());
        window_element.set_class_name("window");
        window_element.set_inner_html(&html);

        drag::make_draggable(&window_element);
        self.container.append_child(&window_element).unwrap();

        self.window_pool.push(window);
        self.window_pool.last().unwrap()
    }
}
