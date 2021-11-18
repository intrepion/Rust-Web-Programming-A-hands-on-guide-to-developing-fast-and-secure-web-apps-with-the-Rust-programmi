#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_print_given_run() {
        let expected = ();

        let some_coordinate = Coordinate { x: 1, y: 1 };

        let actual = print(some_coordinate);

        assert_eq!(expected, actual);
    }
}

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: i8,
    y: i8,
}

fn print(point: Coordinate) {
    println!("{} {}", point.x, point.y);
}

fn main() {
    let test = Coordinate { x: 1, y: 2 };
    print(test);
    println!("{}", test.x);
    println!("{:?}", test);
}
