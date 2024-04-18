//! # Riglet 
//!
//! Riglet is a Rust port of Figlet
//! # Examples
//!
//! ```
//! use riglet::riglet;
//!
//! fn main() {
//!    let ascii = riglet::convert(String::from("Abc Def 123 456"));
//!    riglet::print_ascii(ascii);
//! }
//! ```
pub mod riglet {

    use std::fs::File;
    use std::io::Read;
    use std::collections::HashMap;
    use std::collections::BTreeMap;
    /// Converts text to ascii letter that you can print out. It uses the standard font from figlet
    /// # Examples
    ///
    /// ```
    /// riglet::convert(String::from("abcdef123456"));
    /// ```
    pub fn convert(text: String) -> BTreeMap<i32, String> {
        let mut data_file = File::open("standard.flf").unwrap();
        let mut file_content = String::new();
        data_file.read_to_string(&mut file_content).unwrap();
        let parts = file_content.split("@@");
        let mut collection: Vec<&str> = parts.collect();
        collection.remove(0);

        let char_to_ascii = setup_hash_map(collection);

        return string_to_ascii(text, char_to_ascii)
    }


    /// Setup the hash map with the char and string
    fn setup_hash_map(collection: Vec<&str>) -> HashMap<char, String> {

        let list_of_numbers = [
            '0', 
            '1', 
            '2',
            '3',
            '4',
            '5',
            '6',
            '7',
            '8',
            '9',
        ];

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

        let list_of_small_chars = [
            'a', 
            'b', 
            'c',
            'd',
            'e',
            'f',
            'g',
            'h',
            'i',
            'j',
            'k',
            'l',
            'm',
            'n',
            'o',
            'p',
            'q',
            'r',
            's',
            't',
            'u',
            'v',
            'w',
            'x',
            'y',
            'z',
        ];

        let mut char_to_ascii = HashMap::new();

        let mut start_index = 15;
        for hash_char in list_of_numbers {
            char_to_ascii.insert(
                hash_char, 
                collection.get(start_index).unwrap().replace("@", "")
            );

            start_index += 1;
        }
        
        let mut start_index = 32;
        for hash_char in list_of_chars {
            char_to_ascii.insert(
                hash_char, 
                collection.get(start_index).unwrap().replace("@", "")
            );

            start_index += 1;
        }

        let mut start_index = 64;
        for hash_char in list_of_small_chars {
            char_to_ascii.insert(
                hash_char, 
                collection.get(start_index).unwrap().replace("@", "")
            );

            start_index += 1;
        }


        char_to_ascii.insert(
            ' ', 
            collection.get(101).unwrap().replace("$@", "  ").replace("$", "  ").replace("160  NO-BREAK SPACE", "  ")
        );

        return char_to_ascii
    }

    /// Your inputed string to ascii letters
    fn string_to_ascii(text: String, char_to_ascii: HashMap<char, String>) -> BTreeMap<i32, String> {
        let mut print_ascii = Vec::new();
        
        for character in text.chars() {
            print_ascii.push(char_to_ascii.get(&character).unwrap());
        }

        let mut map_ascii = BTreeMap::from([
            (0, String::from("")),
            (1, String::from("")),
            (2, String::from("")),
            (3, String::from("")),
            (4, String::from("")),
            (5, String::from("")),
            (6, String::from("")),
        ]);

        for string_ascii in &print_ascii {
            let mut test_number = 0;

            for string in string_ascii.split("\n") {
                if test_number <= 6 {
                   map_ascii.get_mut(&test_number).unwrap().push_str(string);
                }

                test_number += 1;
            }
        }

        map_ascii
    }


    /// Prints the Ascii
    /// # Examples
    ///
    /// ```
    ///  riglet::print_ascii(ascii);
    /// ```
    pub fn print_ascii(to_print_ascii: BTreeMap<i32, String>) -> () {
        for (_c, string) in to_print_ascii {
            println!("{}", string)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_convert() {
            let args = String::from("T");
            let anwser = convert(args);
            
            assert_eq!(Some(&String::from("")), anwser.get(&0));
            assert_eq!(Some(&String::from("  _____ ")), anwser.get(&1));
            assert_eq!(Some(&String::from(" |_   _|")), anwser.get(&2));
            assert_eq!(Some(&String::from("   | |  ")), anwser.get(&3));
            assert_eq!(Some(&String::from("   | |  ")), anwser.get(&4));
            assert_eq!(Some(&String::from("   |_|  ")), anwser.get(&5));
        }

        #[test]
        fn test_convert_space() {
            let args = String::from(" ");
            let anwser = convert(args);
            
            assert_eq!(Some(&String::from("")), anwser.get(&0));
            assert_eq!(Some(&String::from("  ")), anwser.get(&1));
            assert_eq!(Some(&String::from("  ")), anwser.get(&2));
            assert_eq!(Some(&String::from("  ")), anwser.get(&3));
            assert_eq!(Some(&String::from("  ")), anwser.get(&4));
            assert_eq!(Some(&String::from("  ")), anwser.get(&5));
        }

        #[test]
        fn test_string_to_ascii() {
            let args = String::from("T");

            let anwser = string_to_ascii(args, HashMap::from([('T', String::from("  _____ 
     |_   _|
       | |  
       | |  
       |_|  
          "))])
                                         );
            
            assert_eq!(Some(&String::from("  _____ ")), anwser.get(&0));
            assert_eq!(Some(&String::from(" |_   _|")), anwser.get(&1));
            assert_eq!(Some(&String::from("   | |  ")), anwser.get(&2));
            assert_eq!(Some(&String::from("   | |  ")), anwser.get(&3));
            assert_eq!(Some(&String::from("   |_|  ")), anwser.get(&4));
        }
    }
}
