fn main() {
    let one: String = String::from("one");
    {
        println!("{}", one);
        let two: String = String::from("two");
    }
    println!("{}", one);
    // println!("{}", two);
}
