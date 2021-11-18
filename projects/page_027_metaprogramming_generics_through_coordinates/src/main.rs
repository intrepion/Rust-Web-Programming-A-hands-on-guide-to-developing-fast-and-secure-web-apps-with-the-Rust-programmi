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

struct Coordinate <T> {
    x: T,
    y: T,
}

fn some_function() {
    let one = Coordinate{x: 50, y: 50};
    println!("one: ({}, {})", one.x, one.y);
    let two = Coordinate{x: 500, y: 500};
    println!("two: ({}, {})", two.x, two.y);
    let three = Coordinate{x: 5.6, y: 5.6};
    println!("three: ({}, {})", three.x, three.y);
}

fn main() {
    some_function();
}
