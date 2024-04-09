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

    let mut char_to_ascii = HashMap::new();
    char_to_ascii.insert('#', collection.get(2).unwrap().replace("@", ""));
    char_to_ascii.insert('$', collection.get(3).unwrap().replace("@", ""));

    let mut result = String::from("");
    
    for character in text.chars() {
        result.push_str(char_to_ascii.get(&character).unwrap());
    }

    return result;
}
