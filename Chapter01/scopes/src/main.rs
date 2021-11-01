fn alter_number(number: &mut String) {
    number.push("!".chars().next().unwrap());
}

fn print_number(number: &String) {
    println!("{}", number);
}

fn main() {
    let mut one: String = String::from("one");
    print_number(&one);
    alter_number(&mut one);
    println!("{}", one);
}
