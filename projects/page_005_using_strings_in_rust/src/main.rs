#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_print_string_when_run() {
        let expected = ();
        let actual = print_string("Hello, World!".to_string());

        assert_eq!(expected, actual);
    }
}

fn print_string(input_string: String) {
    println!("{}", input_string);
}

fn main() {
    let test_string = String::from("Hello, World!");

    print_string(test_string);
}
