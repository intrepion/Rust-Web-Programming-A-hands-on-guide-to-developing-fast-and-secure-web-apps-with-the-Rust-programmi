struct Coordinate {
    x: i8,
    y: i8,
}

fn print(point: Coordinate) {
    println!("{} {}", point.x, point.y);
}

fn main() {
    let test = Coordinate {x: 1, y: 2};
    print(test);
    println!("{}", test.x);
}
