use riglet::riglet;

fn main() {
    let ascii = riglet::convert(String::from("|{}"));
    riglet::print_ascii(ascii);
}
