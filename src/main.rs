use askama::Template;

#[derive(Template)]
#[template(path = "root.html")]
struct Root {
    windows: String,
    icons: String,
}

#[derive(Template)]
#[template(path = "window.html")]
struct Window {
    title: String,
    id: String,
}

#[derive(Template)]
#[template(path = "icon.html")]
struct Icon {
    window_id: String,
    title: String,
    text: String,
}

fn main() {}
