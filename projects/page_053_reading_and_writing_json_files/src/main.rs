#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_some_function_given_run() {
        let expected = ();
        let actual = some_function();

        assert_eq!(expected, actual);
    }
}

mod state;

use std::env;
use state::{read_file, write_to_file};
use serde_json::value::Value;
use serde_json::{json, Map};

fn some_function() {}

fn main() {
    some_function();
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Map<String, Value> = read_file("./state.json");
    println!("{:?}", state);

    state.insert(title.to_string(), json!(status));
    write_to_file("./state.json", &mut state);
}
