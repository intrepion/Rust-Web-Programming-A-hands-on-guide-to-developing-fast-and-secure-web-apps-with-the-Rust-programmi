#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_alter_text_when_run() {

        let expected = "1!".to_string();
        let mut actual = "1".to_string();

        alter_text(&mut actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeed_alter_number_when_run() {
        let expected = 2;
        let mut actual = 1;

        alter_number(&mut actual);

        assert_eq!(expected, actual);
    }
}

fn alter_text(text: &mut String) {

    text.push("!".chars().next().unwrap());
}

fn print_text(text: &String) {

    println!("{}", text);
}

fn alter_number(number: &mut i8) {
    *number += 1
}

fn print_number(number: i8) {
    println!("{}", number);
}

fn main() {

    let mut foo: String = String::from("foo");
    {
        println!("{}", foo);
        let bar: String = String::from("bar");
        println!("{}", bar);
    }
    print_text(&foo);
    alter_text(&mut foo);
    println!("{}", foo);

    let mut one: i8 = 1;
    {
        println!("{}", one);
        let two: i8 = 2;
        println!("{}", two);
    }
    print_number(one);
    alter_number(&mut one);
    println!("{}", one);
}
