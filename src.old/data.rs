use web_sys::{Document, Element};
mod passdle;

pub const WINDOWS: [WindowData; 3] = [
    WindowData {
        name: "Passdle",
        id: "passdle",
        html_content: include_str!("data/passdle/passdle.html"),
        icon_url: "/icons/passdle.png",
        callback: Some(passdle::start),
    },
    WindowData {
        name: "Test",
        id: "test",
        html_content: "Hiya!",
        icon_url: "/icons/test.png",
        callback: None,
    },
    WindowData {
        name: "Test",
        id: "test2",
        html_content: "Hiya!",
        icon_url: "/icons/test.png",
        callback: None,
    },
];

// Structs
pub struct WindowData<'a> {
    /// id: To use the value from this field, use the `{{ window_id }}` placeholder
    name: &'a str,
    id: &'a str,

    /// html_content: Any ID/class in the content must start with the ID set above
    /// icon_url: Must point to an image in `/html`, ex: `/icons/image.png`
    html_content: &'a str,
    icon_url: &'a str,

    /// callback: Allows for custom functionality for advanced windows
    callback: Option<fn(Element, &Document) -> ()>,
}

impl<'a> WindowData<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn id(&self) -> &'a str {
        self.id
    }

    pub fn html_content(&self) -> &'a str {
        self.html_content
    }

    pub fn icon_url(&self) -> &'a str {
        self.icon_url
    }

    pub fn callback(&self, element: Element, document: &Document) {
        if let Some(callback) = self.callback {
            callback(element, document);
        }
    }
}
