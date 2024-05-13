use riglet::riglet;

fn main() {
    let ascii = riglet::convert(String::from("This is a test"));
    riglet::print_ascii(ascii);
}
