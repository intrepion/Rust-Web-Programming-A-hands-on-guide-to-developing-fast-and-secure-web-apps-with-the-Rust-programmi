#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handle_check_new_project_with_no_environment_arguments() {
        let expected = "Missing environment arguments";
        let actual = check_new_project(vec!["new_project".to_string()]).unwrap_err();
        assert_eq!(expected, actual);
    }
}

fn check_new_project(_arguments: Vec<String>) -> Result<(), &'static str> {
    return Err("Missing environment arguments");
}

fn main() {
    use std::env;

    let result = check_new_project(env::args().collect());
    match result {
        Ok(o) => println!("okay: {:?}", o),
        Err(e) => println!("error: {:?}", e),
    }
}
