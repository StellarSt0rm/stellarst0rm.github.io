use include_dir::File;

// Inverted until I figure out a better way to do the parsing
const SDO_SECTIONS: [&str; 3] = ["Content:", "Icon:", "Title:"];

pub fn parse<'a>(files: impl Iterator<Item = &'a File<'a>>) -> Vec<Vec<String>> {
    let files = files.collect::<Vec<_>>();
    let mut parsed_files: Vec<Vec<String>> = Vec::with_capacity(files.len());

    for file in files {
        // Skip non .sdo files
        if file.path().extension().unwrap() != "sdo" {
            continue;
        }

        let mut content = file.contents_utf8().unwrap();
        let mut parsed_sections: Vec<String> = Vec::with_capacity(SDO_SECTIONS.len());

        for section in SDO_SECTIONS {
            let (remaining, parsed_section) = content.split_once(section).unwrap();

            parsed_sections.push(parsed_section.trim().to_string());
            content = remaining;
        }

        parsed_sections.reverse();
        parsed_files.push(parsed_sections);
    }

    parsed_files
}
