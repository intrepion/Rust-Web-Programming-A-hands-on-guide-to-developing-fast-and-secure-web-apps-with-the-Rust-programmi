use std::collections::HashMap;

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_some_function_when_run() {

        let expected = ();
        let actual = some_function();

        assert_eq!(expected, actual);
    }
}

fn some_function() {
    let mut general_map: HashMap<&str, i8> =
        HashMap::new();

    general_map.insert("test", 25);

    let _outcome: Option<&i8> = general_map.get("test");

    match general_map.get("testing") {
        None => {
            match general_map.get("test") {
                None => println!("Both testing and test failed"),
                Some(result) => println!("testing failed but test is: {}", result),
            }
        },
        Some(result) => println!("Here is the result: {}", result),
    }
}

fn main() {

    some_function();
}
