use web_sys::{Document, Element};
mod passdle;

pub const WINDOWS: [WindowData; 1] = [WindowData {
    name: "Passdle",
    html_content: include_str!("data/passdle/passdle.html"),
    icon_url: "/icons/passdle.png",
    callback: Some(passdle::start),
}];

// Structs
/// Callback field is used for more advanced windows with custom functionality.
/// eg. Games or very dynamic windows.
/// Note: Any id/class in the content must start with '<name>_' to avoid conflicts.
pub struct WindowData<'a> {
    name: &'a str,
    html_content: &'a str,
    icon_url: &'a str,
    callback: Option<fn(Element, &Document) -> ()>,
}

impl<'a> WindowData<'a> {
    pub fn name(&self) -> &'a str {
        self.name
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
