use riglet::riglet;

fn main() {
    let ascii = riglet::convert(String::from("Abc Def 123 456"));
    riglet::print_ascii(ascii);
}
