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
        }
    }

    fn with_thought(mut self, thought: String) -> Human {
        self.current_thought = thought;
        return self
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
