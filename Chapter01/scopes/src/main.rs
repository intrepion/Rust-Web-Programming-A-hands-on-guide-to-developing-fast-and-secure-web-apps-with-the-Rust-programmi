fn alter_number(number: &mut i8) {
    *number += 1
}

fn print_number(number: i8) {
    println!("{}", number);
}

fn main() {
    let mut one: i8 = 1;
    print_number(one);
    alter_number(&mut one);
    println!("{}", one);
}
