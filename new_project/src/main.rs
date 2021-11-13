#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn error_when_check_new_project_with_missing_page_number() {

        let expected = format!("Missing page number.\n{}", USAGE);

        let arguments = vec![
            "new_project".to_string(),
        ];

        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn error_when_check_new_project_missing_last_page() {

        let expected = "Missing last page.";

        let arguments = vec![
            "new_project".to_string(),
            "1".to_string(),
        ];

        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn error_when_check_new_project_with_an_english_page_number() {

        let expected = "Invalid page number.";

        let arguments = vec![
            "new_project".to_string(),
            "one".to_string(),
            "256".to_string(),
            "hello_world".to_string(),
        ];

        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn error_when_check_new_project_with_an_english_last_page() {

        let expected = "Invalid last page.";

        let arguments = vec![
            "new_project".to_string(),
            "1".to_string(),
            "ninety".to_string(),
            "hello_world".to_string(),
        ];

        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn error_when_check_new_project_with_missing_project_name() {

        let expected = "Missing project name.";

        let arguments = vec![
            "new_project".to_string(),
            "1".to_string(),
            "256".to_string(),
        ];

        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn error_when_check_new_project_with_empty_project_name() {

        let expected = "Empty project name.";

        let arguments = vec![
            "new_project".to_string(),
            "1".to_string(),
            "256".to_string(),
            " ".to_string(),
        ];

        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeed_when_check_new_project_happy_path() {

        let expected = Project {
            last_page: 256,
            name: "hello".to_string(),
            page_number: 1,
        };

        let arguments = vec![
            "new_project".to_string(),
            "1".to_string(),
            "256".to_string(),
            "hello".to_string(),
        ];

        let actual = check_new_project(arguments).unwrap();

        assert_eq!(expected, actual);
    }
}

static USAGE: &str = "Usage: new_project <page_number> <last_page> <project_name>";

#[derive(Debug, PartialEq)]
struct Project {
    last_page: u32,
    name: String,
    page_number: u32,
}

fn check_new_project(arguments: Vec<String>) -> Result<Project, String> {

    if arguments.len() == 1
    {
        return Err(format!("Missing page number.\n{}", USAGE));
    }

    if arguments.len() == 2
    {
        return Err("Missing last page.".to_string());
    }

    if arguments.len() == 3
    {
        return Err("Missing project name.".to_string());
    }

    if arguments[3].trim().eq("")
    {
        return Err("Empty project name.".to_string());
    }

    return match arguments[1].parse::<u32>() {

        Ok(page_number) => match arguments[2].parse::<u32>() {
            
            Ok(last_page) => Ok(Project {
                last_page: last_page,
                name: arguments[3].to_string(),
                page_number: page_number,
            }),
            Err(_) => Err("Invalid last page.".to_string()),
        },
        Err(_) => Err("Invalid page number.".to_string()),
    }
}

fn main() {

    use std::env;

    println!("This is {}", USAGE);

    let result = check_new_project(env::args().collect());

    match result {

        Ok(project) => println!("okay: {:?}", project),
        Err(error) => println!("error: {:?}", error),
    }
}
