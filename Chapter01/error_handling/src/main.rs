fn error_check(check: bool) -> Result<i8, &'static str> {
    if check == true {
        Err("this is an error")
    } else {
        Ok(1)
    }
}

fn main() {
    let result: i8 = error_check(false).unwrap();
    println!("{}", result);
}
