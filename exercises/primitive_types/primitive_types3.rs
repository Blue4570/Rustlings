// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!


fn main() {
    let a = [0;100];
    println!("Array 'a' size is {}", a.len());

    for (i, v) in a.iter().enumerate() {
        println!("Index {}, value: {:?}", i, v);
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
