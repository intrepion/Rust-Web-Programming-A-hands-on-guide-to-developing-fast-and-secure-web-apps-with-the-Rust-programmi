#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_alter_number_when_run() {

        let expected = "1!".to_string();
        let mut actual = "1".to_string();

        alter_number(&mut actual);

        assert_eq!(expected, actual);
    }
}

fn alter_number(number: &mut String) {

    number.push("!".chars().next().unwrap());
}

fn print_number(number: &String) {

    println!("{}", number);
}

fn main() {

    let mut one: String = String::from("one");
    {
        println!("{}", one);
        let two: String = String::from("two");
        println!("{}", two);
    }
    print_number(&one);
    alter_number(&mut one);
    println!("{}", one);
}
