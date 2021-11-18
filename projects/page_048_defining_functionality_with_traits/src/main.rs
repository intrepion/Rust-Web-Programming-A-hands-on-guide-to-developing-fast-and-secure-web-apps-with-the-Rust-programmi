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

mod to_do;

use to_do::structs::traits::create::Create;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn some_function() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "washing");

    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!(
            "it's a done item with the title: {}",
            item.super_struct.title
        ),
    }
}

fn main() {
    some_function();
}
