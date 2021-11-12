#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handle_check_new_project_with_missing_page_number() {
        let expected = "Missing page number.";
        let arguments = vec![
            "new_project".to_string(),
        ];
        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn handle_check_new_project_with_an_english_page_number() {
        let expected = "Invalid page number.";
        let arguments = vec![
            "new_project".to_string(),
            "one".to_string(),
            "hello_world".to_string(),
        ];
        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn handle_check_new_project_with_missing_project_name() {
        let expected = "Missing project name.";
        let arguments = vec![
            "new_project".to_string(),
            "1".to_string(),
        ];
        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn handle_check_new_project_with_empty_project_name() {
        let expected = "Empty project name.";
        let arguments = vec![
            "new_project".to_string(),
            "1".to_string(),
            " ".to_string(),
        ];
        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn handle_check_new_project_happy_path() {
        let expected = 1;  
        let arguments = vec![
            "new_project".to_string(),
            "1".to_string(),
            "hello".to_string(),
        ];
        let actual = check_new_project(arguments).unwrap();

        assert_eq!(expected, actual);
    }
}

fn check_new_project(arguments: Vec<String>) -> Result<u32, &'static str> {
    if arguments.len() == 1
    {
        return Err("Missing page number.");
    }

    if arguments.len() == 2
    {
        return Err("Missing project name.");
    }

    if arguments[2].trim().eq("")
    {
        return Err("Empty project name.");
    }

    return match arguments[1].parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err("Invalid page number."),
    }
}

fn main() {
    use std::env;

    let result = check_new_project(env::args().collect());
    match result {
        Ok(o) => println!("okay: {:?}", o),
        Err(e) => println!("error: {:?}", e),
    }
}
