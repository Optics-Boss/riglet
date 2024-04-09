use std::fs::File;
use std::io::Read;

pub fn convert(text: String) -> String {
    let mut data_file = File::open("./src/standard.flf").unwrap();
    let mut file_content = String::new();
    data_file.read_to_string(&mut file_content).unwrap();
    let parts = file_content.split("@@");
    let mut collection: Vec<&str> = parts.collect();
    collection.remove(0);

    println!("{}", collection.get(2).unwrap().replace("@", ""));
    return String::from("Test");
}
