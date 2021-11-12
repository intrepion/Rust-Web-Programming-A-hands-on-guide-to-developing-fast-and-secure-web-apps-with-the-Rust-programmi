#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handle_check_new_project_with_no_environment_arguments() {
        let expected = "Missing page number.";
        let arguments = vec![
            "new_project".to_string(),
        ];
        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn handle_check_new_project_with_an_invalid_page_number() {
        let expected = "Invalid page number.";
        let arguments = vec![
            "new_project".to_string(),
            "one".to_string(),
        ];
        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }
}

fn check_new_project(arguments: Vec<String>) -> Result<(), &'static str> {
    if arguments.len() == 1
    {
        return Err("Missing page number.");
    }

    return Err("Invalid page number.");
}

fn main() {
    use std::env;

    let result = check_new_project(env::args().collect());
    match result {
        Ok(o) => println!("okay: {:?}", o),
        Err(e) => println!("error: {:?}", e),
    }
}
