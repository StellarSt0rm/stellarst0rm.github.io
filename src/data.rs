use web_sys::Element;

mod passdle;

pub const WINDOWS: [WindowData; 1] = [WindowData {
    name: "Passdle",
    content: include_str!("data/html/passdle.html"),
    icon_url: "/icons/passdle.png",
    callback: Some(passdle::start),
}];

// Structs
/// Callback field is used for more advanced windows with custom functionality.
/// eg. Games or very dynamic windows.
pub struct WindowData<'a> {
    name: &'a str,
    content: &'a str,
    icon_url: &'a str,
    callback: Option<fn(Element) -> ()>,
}

impl<'a> WindowData<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn content(&self) -> &'a str {
        self.content
    }

    pub fn icon_url(&self) -> &'a str {
        self.icon_url
    }

    pub fn callback(&self, element: Element) {
        if let Some(callback) = self.callback {
            callback(element);
        }
    }
}
