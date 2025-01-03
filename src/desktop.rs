use web_sys::{Document, Element};

pub struct Desktop {
    container: Element,
    document: Document,
    window_pool: Vec<Window>,
    pub active_window: Option<u32>,
}

pub struct Window {
    pub layer: usize,
    pub name: String,
    pub content: String,
    pub icon_url: String,
}

impl Desktop {
    pub fn new(container: Element) -> Desktop {
        let document = web_sys::window().unwrap().document().unwrap();

        Desktop {
            container,
            document,
            window_pool: Vec::new(),
            active_window: None,
        }
    }

    pub fn new_window(&mut self, name: String, content: String, icon_url: String) -> &Window {
        let window = Window {
            layer: 0,
            name,
            content,
            icon_url,
        };

        let text = self.document.create_element("p").unwrap();
        text.set_text_content(Some(&window.name));

        self.container.append_child(&text).unwrap();

        self.window_pool.push(window);
        self.window_pool.last().unwrap()
    }
}
