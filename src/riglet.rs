use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn convert(text: String) -> String {
    let mut data_file = File::open("./src/standard.flf").unwrap();
    let mut file_content = String::new();
    data_file.read_to_string(&mut file_content).unwrap();
    let parts = file_content.split("@@");
    let mut collection: Vec<&str> = parts.collect();
    collection.remove(0);

    let char_to_ascii = setup_hash_map(collection);

    return string_to_ascii(text, char_to_ascii)
}

fn setup_hash_map(collection: Vec<&str>) -> HashMap<char, String> {
    let list_of_chars = [
        'A', 
        'B', 
        'C',
        'D',
        'E',
        'F',
        'G',
        'H',
        'I',
        'J',
        'K',
        'L',
        'M',
        'N',
        'O',
        'P',
        'Q',
        'R',
        'S',
        'T',
        'U',
        'V',
        'W',
        'X',
        'Y',
        'Z',
    ];
    
    let mut start_index = 32;

    
    let mut char_to_ascii = HashMap::new();

    for hash_char in list_of_chars {
        char_to_ascii.insert(
            hash_char, 
            collection.get(start_index).unwrap().replace("@", "")
        );

        start_index += 1;
    }
    return char_to_ascii
}

fn string_to_ascii(text: String, char_to_ascii: HashMap<char, String>) -> String {
    let mut result = String::from("");
    
    for character in text.chars() {
        result.push_str(&char_to_ascii.get(&character).unwrap());
    }

    return result
}


