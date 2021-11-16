#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_some_function_when_run() {
        let expected = ();
        let actual = some_function();

        assert_eq!(expected, actual);
    }
}

fn some_function() {
    let _number: u8 = 255;

    let _float: f32 = 20.6;

    let _x = 1u8;
}

fn main() {
    some_function();
}
