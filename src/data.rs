use web_sys::Element;

pub const WINDOWS: [WindowData; 1] = [WindowData {
    name: "Callback Test",
    content: "This should be overwritten!",
    icon_url: "/aaaa",
    callback: Some(test_function),
}];

fn test_function(element: Element) {
    element
        .query_selector(".content")
        .unwrap()
        .unwrap()
        .set_text_content(Some("Callback test works!"));
    web_sys::console::log_2(&"Window:".into(), &element.into());
}

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
