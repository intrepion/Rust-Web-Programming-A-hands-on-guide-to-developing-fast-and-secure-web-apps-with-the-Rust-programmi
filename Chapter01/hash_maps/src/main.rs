use std::collections::HashMap;

fn main() {
    let mut general_map: HashMap<&str, i8> = HashMap::new();
    general_map.insert("test", 25);
    match general_map.get("test") {
        None => println!("it failed"),
        Some(result) => println!("here is the result: {}", result),
    }
}
