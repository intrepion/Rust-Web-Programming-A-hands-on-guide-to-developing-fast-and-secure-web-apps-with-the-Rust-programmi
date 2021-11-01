fn main() {
    let one;
    {
        let two: i8 = 2;
        one = &two
    } // -----------------------> two lifetime stops here
    println!("r: {}", one);
}
