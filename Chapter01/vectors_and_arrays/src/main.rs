fn main() {
    let int_array: [i32; 3] = [1, 2, 3];

    for i in int_array.iter() {
        println!("{}", i);
    }

    let mut str_vector: Vec<&str> = vec!["one", "two", "there"];

    for i in str_vector.iter() {
        println!("{}", i);
    }

    let second_int_array: [i32; 3] = [1, 2, 3];
    let _two = second_int_array[1];

    str_vector.push("four");
}
