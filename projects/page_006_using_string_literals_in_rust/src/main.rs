#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_print_string_when_run() {

        let expected = ();
        let actual = print_string("Hello, World!");

        assert_eq!(expected, actual);
    }
}

fn print_string(input_string: &str) {

    println!("{}", input_string);
}

fn main() {

    let test_string = &"Hello, World!";

    print_string(test_string);
}
