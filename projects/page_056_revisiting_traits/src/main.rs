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
mod to_do;

use serde_json::value::Value;
use serde_json::{json, Map};
use state::{read_file, write_to_file};
use std::env;
use to_do::structs::traits::create::Create;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn some_function() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Map<String, Value> = read_file("./state.json");
    println!("{:?}", state);

    state.insert(title.to_string(), json!(status));
    write_to_file("./state.json", &mut state);

    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory(status, title);

    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(
            &item.super_struct.title,
            &item.super_struct.status,
            &mut state,
        ),
        ItemTypes::Done(item) => println!(
            "it's a done item with the title: {}",
            item.super_struct.title
        ),
    }
    some_function();
}
