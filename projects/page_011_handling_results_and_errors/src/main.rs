#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_error_check_when_false() {

        let expected = Ok(1);
        let actual = error_check(false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn error_error_check_when_true() {

        let expected = Err("this is an error");
        let actual = error_check(true);

        assert_eq!(expected, actual);
    }
}

fn error_check(check: bool) -> Result<i8, &'static str> {

    if check == true {

        Err("this is an error")
    } else {

        Ok(1)
    }
}

fn describe_result(result: Result<i8, &'static str>) {

    match result {

        Ok(x) => println!("it's a result of: {}", x),
        Err(x) => println!("{}", x),
    }
}

fn main() {

    let result: Result<i8, &'static str> = error_check(true);
    describe_result(result);

    if result.is_err() {
        panic!("throwing some error");
    }
}
