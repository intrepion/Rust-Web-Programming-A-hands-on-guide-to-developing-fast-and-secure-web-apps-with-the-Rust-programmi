fn print_number(number: String) {
    println!("{}", number);
}

fn main() {
    let one: String = String::from("one");
    print_number(one);
    println!("{}", one);
}
