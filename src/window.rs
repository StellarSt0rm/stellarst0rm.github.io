pub fn create_window(document: &web_sys::Document, title: &str, icon: &str) {
    let container = document
        .get_element_by_id("window_container")
        .expect("Couldnt find the #window_container element");

    let window_element = document.create_element("div").unwrap();
    let title_element = document.create_element("p").unwrap();
    let window_id = format!("window-{}", container.children().length() + 1);

    window_element.set_id(&window_id);
    window_element.set_class_name("window");
    window_element.set_attribute("popover", "").ok();

    title_element.set_class_name("title");
    title_element.set_text_content(Some(title));

    // Superglue
    window_element.append_child(&title_element).unwrap();
    window_element
        .append_child(&create_window_controls(document, &window_id))
        .unwrap();

    create_app_icon(document, &window_id, title, icon);

    container.append_child(&window_element).unwrap();
}

fn create_app_icon(
    document: &web_sys::Document,
    window_id: &str,
    title: &str,
    icon: &str,
) -> web_sys::Element {
    let app_icon = document.create_element("button").unwrap();
    let text_icon = document.create_element("div").unwrap();
    let label = document.create_element("div").unwrap();

    app_icon.set_class_name("icon");
    app_icon.set_attribute("popovertarget", window_id).ok();

    text_icon.set_text_content(Some(icon));
    label.set_text_content(Some(title));

    app_icon.append_child(&text_icon).unwrap();
    app_icon.append_child(&label).unwrap();

    // Stiching
    document
        .get_element_by_id("icon_container")
        .expect("Couldnt find the #icon_container element")
        .append_child(&app_icon)
        .unwrap();

    app_icon
}

fn create_window_controls(document: &web_sys::Document, window_id: &str) -> web_sys::Element {
    let controls = document.create_element("div").unwrap();
    let close_button = document.create_element("button").unwrap();

    controls.set_class_name("controls");

    close_button.set_class_name("button");
    close_button.set_attribute("popovertarget", window_id).ok();
    close_button.set_text_content(Some("x"));

    // Stiching
    controls.append_child(&close_button).unwrap();

    controls
}
