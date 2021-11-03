use std::collections::HashMap;

enum AllowedData {
    S(String),
    I(i8),
}

struct CustomMap {
    body: HashMap<String, AllowedData>
}

impl CustomMap {
    fn new() -> CustomMap {
        return CustomMap { body: HashMap::new() }
    }

    fn get(&self, key: &str) -> &AllowedData {
        return self.body.get(key).unwrap()
    }

    fn insert(&mut self, key: &str, value: AllowedData) -> () {
        self.body.insert(key.to_string(), value);
    }

    fn display(&self, key: &str) -> () {
        match self.get(key) {
            AllowedData::I(value) => println!("{}", value),
            AllowedData::S(value) => println!("{}", value),
        }
    }
}

fn main() {
    // defining a new hash map
    let mut map = CustomMap::new();

    // inserting two different types of data
    map.insert("test", AllowedData::I(8));
    map.insert("testing", AllowedData::S("test value".to_string()));

    // displaying the data
    map.display("test");
    map.display("testing");
}
