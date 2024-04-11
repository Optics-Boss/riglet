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
    let mut print_ascii = Vec::new();
    let result = String::from("");
    
    for character in text.chars() {
        print_ascii.push(char_to_ascii.get(&character).unwrap());
    }

    let mut string_test = String::from("");
    let mut string_test_2 = String::from("");
    let mut string_test_3 = String::from("");
    let mut string_test_4 = String::from("");
    let mut string_test_5 = String::from("");
    let mut string_test_6 = String::from("");

    for string_ascii in &print_ascii {
        let mut test_number = 0;

        for string in string_ascii.split("\n") {
            if test_number == 0 {
                string_test.push_str(string)
            }
            if test_number == 1 {
                string_test_2.push_str(string)
            }
            if test_number == 2 {
                string_test_3.push_str(string)
            }
            if test_number == 3 {
                string_test_4.push_str(string)
            }
            if test_number == 4 {
                string_test_5.push_str(string)
            }
            if test_number == 5 {
                string_test_6.push_str(string)
            }

            test_number += 1;
        }
    }

    println!("{}", string_test);
    println!("{}", string_test_2);
    println!("{}", string_test_3);
    println!("{}", string_test_4);
    println!("{}", string_test_5);
    println!("{}", string_test_6);

    return result
}


