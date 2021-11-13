#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn error_check_new_project_when_missing_page_number() {

        let expected = format!("Missing page number.\n{}", USAGE);

        let arguments = vec![
            "new_project".to_string(),
        ];

        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn error_check_new_project_when_missing_last_page() {

        let expected = format!("Missing last page.\n{}", USAGE);

        let arguments = vec![
            "new_project".to_string(),
            "1".to_string(),
        ];

        let actual = check_new_project(arguments).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn error_check_new_project_when_english_page_number() {

        let expected = format!("Invalid page number.\n{}", USAGE);

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
    fn error_check_new_project_when_english_last_page() {

        let expected = format!("Invalid last page.\n{}", USAGE);

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
    fn error_check_new_project_when_missing_project_name() {

        let expected = format!("Missing project name.\n{}", USAGE);

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

        let expected = format!("Empty project name.\n{}", USAGE);

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
        return Err(format!("Missing last page.\n{}", USAGE));
    }

    if arguments.len() == 3
    {
        return Err(format!("Missing project name.\n{}", USAGE));
    }

    if arguments[3].trim().eq("")
    {
        return Err(format!("Empty project name.\n{}", USAGE));
    }

    return match arguments[1].parse::<u32>() {

        Ok(page_number) => match arguments[2].parse::<u32>() {
            
            Ok(last_page) => Ok(Project {
                last_page: last_page,
                name: arguments[3].to_string(),
                page_number: page_number,
            }),
            Err(_) => Err(format!("Invalid last page.\n{}", USAGE)),
        },
        Err(_) => Err(format!("Invalid page number.\n{}", USAGE)),
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
