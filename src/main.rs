mod silly_data;
mod templates;

use std::io::Write;

use include_dir::{Dir, include_dir};
static TEMPLATES_DATA: Dir<'_> = include_dir!("templates/data");

fn main() {
    let data_files = TEMPLATES_DATA.files();
    let parsed_files = silly_data::parse_sdo(data_files);

    println!("Parsed .sdo files.");

    // Template the HTML
    let mut templated_windows: Vec<String> = Vec::new();
    let mut templated_icons: Vec<String> = Vec::new();

    for (i, application) in parsed_files.iter().enumerate() {
        let title = &application[0];
        let icon = &application[1];
        let content = &application[2];

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

    println!("Templated the HTML.");

    // Write it to processed.html
    let mut file = std::fs::File::create_new("processed.html").unwrap();
    file.write_all(root_template.to_string().as_bytes())
        .unwrap();

    println!("Wrote HTML to `processed.html`");
}
