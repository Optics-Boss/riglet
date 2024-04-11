pub mod riglet;

fn main() {
    let ascii = riglet::convert(String::from("HAHAHAHHAHAHHA"));
    riglet::print_ascii(ascii);
}
