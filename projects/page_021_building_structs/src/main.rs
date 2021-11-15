#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_some_function_given_name_age() {

        let expected = Human {
            name: "Oliver Forral".to_string(),
            age: 38,
            current_thought: "nothing".to_string(),
        };

        let actual = Human::new("Oliver Forral", 38);

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeed_some_function_given_name_age_current_thought() {

        let expected = Human {
            name: "Oliver Forral".to_string(),
            age: 38,
            current_thought: "I want to program in Rust.".to_string(),
        };

        let actual = Human::new("Oliver Forral", 38)
            .with_thought("I want to program in Rust.".to_string());

        assert_eq!(expected, actual);
    }
}

#[derive(Debug, PartialEq)]
struct Human {
    name: String,
    age: i8,
    current_thought: String,
}

impl Human {

    fn new(input_name: &str, input_age: i8) -> Human {

        return Human {
            name: input_name.to_string(),
            age: input_age,
            current_thought: String::from("nothing"),
        };
    }

    fn with_thought(mut self, thought: String) -> Human {

        self.current_thought = thought;

        return self;
    }
}

fn main() {

    let developer = Human::new("Maxwell Flitton", 31);
}
