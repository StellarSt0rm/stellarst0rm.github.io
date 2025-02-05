use web_sys::{window, Document, Element};

pub fn start(window: Element, document: &Document) {
    const ROWS: usize = 6;
    const COLS: usize = 6;

    let input = window
        .query_selector("#passdle_input")
        .unwrap()
        .expect("[passdle] Couldnt find 'passdle_input' element");
    let grid = window
        .query_selector("#passdle_grid")
        .unwrap()
        .expect("[passdle] Couldnt find 'passdle_grid' element");

    input
        .set_attribute("placeholder", &format!("Enter passdle ({COLS} chars)"))
        .unwrap();
    input.set_attribute("maxlength", &COLS.to_string()).unwrap();

    // Prepare grid
    for _ in 0..ROWS {
        let row = document.create_element("div").unwrap();

        for _ in 0..COLS {
            let tile = document.create_element("div").unwrap();
            tile.set_id("tile");
            tile.set_text_content(Some(&generate_password(1)));

            row.append_child(&tile).unwrap();
        }

        grid.append_child(&row).unwrap();
    }

    //content.set_text_content(Some(&generate_password(LENGTH)));
}

fn generate_password(length: usize) -> String {
    //! Generate n length password using only ASCII values

    const CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyz";
    let window = window().unwrap();

    let crypto = window.crypto().expect("No crypto support");
    let mut result = String::with_capacity(length);
    let mut buffer = vec![0u8; length];

    crypto
        .get_random_values_with_u8_array(&mut buffer)
        .expect("Failed to generate random values");

    for &byte in buffer.iter() {
        let index = (byte as usize) % CHARACTERS.len();
        result.push(CHARACTERS.chars().nth(index).unwrap());
    }

    result
}
