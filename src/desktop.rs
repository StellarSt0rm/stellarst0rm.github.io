use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

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
        };

        let text: HtmlElement = self.document.create_element("p").unwrap().unchecked_into();
        text.set_text_content(Some(&window.name));
        text.set_id(&window.id.to_string());
        text.style().set_property("z-index", "0").unwrap();

        self.container.append_child(&text).unwrap();

        self.window_pool.push(window);
        self.window_pool.last().unwrap()
    }
}
