mod sdo_parser;
mod templates;

use include_dir::{Dir, include_dir};
static TEMPLATES_DATA: Dir<'_> = include_dir!("templates/data");

fn main() {
    let data_files = TEMPLATES_DATA.files();
    let parsed_files = sdo_parser::parse(data_files);

    let mut templated_windows: Vec<String> = Vec::new();
    let mut templated_icons: Vec<String> = Vec::new();

    for (i, application) in parsed_files.iter().enumerate() {
        let title = application[0];
        let icon = application[1];
        let content = application[2];

        let id = format!("{}-{}", title.to_lowercase(), i);

        let window_template = templates::Window {
            title,
            content,
            id: id.clone(),
        };
        let icon_template = templates::Icon { title, icon, id };

        templated_windows.push(window_template.to_string());
        templated_icons.push(icon_template.to_string());
    }

    let root_template = templates::Root {
        windows: templated_windows.join("\n"),
        icons: templated_icons.join("\n"),
    };

    println!("{}", root_template);
}
