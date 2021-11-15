#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_human_impl_new_given_name_age() {

        let expected = Human {
            name: "Oliver Forral".to_string(),
            age: 38,
            current_thought: "nothing".to_string(),
        };

        let actual = Human::new("Oliver Forral", 38);

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeed_human_impl_new_given_name_age_current_thought() {

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

    fn speak(&self) -> () {
        
        println!("Hello my name is {} and I'm {} years old.", &self.name, &self.age);
    }
}

fn main() {

    let developer = Human::new("Maxwell Flitton", 31);
    developer.speak();
    println!("currently I'm thinking {}", developer.current_thought);

    let new_developer = Human::new("Grace", 30).with_thought(String::from("I'm Hungry"));
    new_developer.speak();
    println!("currently I'm thinking {}", new_developer.current_thought);
}
