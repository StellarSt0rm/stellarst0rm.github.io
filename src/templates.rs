pub fn window(name: &str, content: &str) -> String {
    let mut template = include_str!("data/window.template").to_string();

    template = template.replace("{{ name }}", name);
    template.replace("{{ content }}", content)
}
