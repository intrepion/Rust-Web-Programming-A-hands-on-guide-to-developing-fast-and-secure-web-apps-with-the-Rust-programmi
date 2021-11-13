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

    use std::process::exit;

    let exit_code = real_main();

    exit(exit_code);
}

fn real_main() -> i32 {

    use std::env;

    let result = check_new_project(env::args().collect());

    match result {

        Ok(project) => {

            println!("okay: {:?}", project);

            create_project(project);
        },
        Err(error) => {

            println!("error: {:?}", error);

            return 1;
        },
    }

    return 0;
}

fn create_project(project: Project) {
    use std::fs;
    use std::iter::repeat;
    use std::process::Command;

    fs::create_dir_all("../projects").unwrap();

    let page_number_digits = ((project.page_number as f64).log(10_f64).trunc() as i32) + 1;
    let last_page_digits = ((project.last_page as f64).log(10_f64).trunc() as i32) + 1;
    let difference = last_page_digits - page_number_digits;

    let project_folder = "page_".to_owned()
        + &repeat("0").take(difference as usize).collect::<String>()
        + &project.page_number.to_string()
        + "_"
        + &project.name;

    Command::new("cargo")
        .current_dir("../projects")
        .arg("new")
        .arg(&project_folder)
        .output()
        .expect("failed to create project");

    let rust_yml_beginning = "name: Rust

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v2
    - name: Build
      run: cd new_project && cargo build --verbose
    - name: Run tests
      run: cd new_project && cargo test --verbose
";

    let github_actions = match fs::read_dir("../projects") {
        Err(_) => vec!["".to_string()],
        Ok(paths) => paths.map(|path| {
            let file_name = path.unwrap().file_name().into_string().unwrap();
            return format!("    - name: Build
      run: cd projects/{} && cargo build --verbose
    - name: Run tests
      run: cd projects/{} && cargo test --verbose
", file_name, file_name);
        }).collect(),
    };

    let mut rust_yml_ending = github_actions
        .iter()
        .map(|s| &**s)
        .collect::<Vec<&str>>();

    rust_yml_ending.sort();

    let rust_yml = rust_yml_beginning.to_owned()
        + &rust_yml_ending.join("");

    fs::write("../.github/workflows/rust.yml", rust_yml).unwrap();

    Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("failed to add changes to staged");

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("\"created project {}\"", project_folder))
        .output()
        .expect("failed to create commit");

    Command::new("git")
        .arg("push")
        .output()
        .expect("failed to push branch");
}
