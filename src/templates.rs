use askama::Template;

#[derive(Template)]
#[template(path = "root.html")]
pub struct Root {
    pub windows: String,
    pub icons: String,
}

#[derive(Template)]
#[template(path = "window.html")]
pub struct Window<'a> {
    pub title: &'a str,
    pub content: &'a str,

    pub id: String,
}

#[derive(Template)]
#[template(path = "icon.html")]
pub struct Icon<'a> {
    pub title: &'a str,
    pub icon: &'a str,

    pub id: String,
}
