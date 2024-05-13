use riglet::riglet;

fn main() {
    let ascii = riglet::convert(String::from("Test 123!"));
    riglet::print_ascii(ascii);
}
