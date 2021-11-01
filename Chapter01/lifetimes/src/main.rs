fn get_highest<'a>(first_number: &'a i8, second_number: &'a i8) -> &'a i8 {
    if first_number > second_number {
        first_number
    } else {
        second_number
    }
}

fn main() {
    let one: i8 = 1;
    {
        let two: i8 = 2;
        let outcome: &i8 = get_highest(&one, &two);
        println!("{}", outcome);
    }
}
