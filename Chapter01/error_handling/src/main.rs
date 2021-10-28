fn error_check(check: bool) -> Result<i8, &'static str> {
    if check == true {
        Err("this is an error")
    } else {
        Ok(1)
    }
}

fn describe_result(result: Result<i8, &'static str>) {
    match result {
        Ok(x) => println!("its a result of: {}", x),
        Err(x) => println!("{}", x),
    }
}

fn main() {
    let result: Result<i8, &'static str> = error_check(true);
    describe_result(result);
}
