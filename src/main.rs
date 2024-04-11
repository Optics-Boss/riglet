pub mod riglet;

fn main() {
    let ascii = riglet::convert(String::from("abcdef123456"));
    riglet::print_ascii(ascii);
}
