/*
* Primitive str = Immutable fixed-length string somewhere in memory
* string = Growable heap-allocated data structure - Use when you need to modify or own string data
*/

pub fn run() {
    // Primitive string
    // let hello = "hello";

    // String
    let mut hello = String::from("Hello");

    // Get length
   println!("Length: {}", hello.len());
    
    // Push (char)
    hello.push(' ');

    // Push string
    hello.push_str("Hi there");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if string is empty
    println!("Is Empty {}", hello.is_empty());

    // Contains
    println!("Contains Hi? {}", hello.contains("Hi"));

    // Replace
    println!("Replace: {}", hello.replace("Hi", "Hello"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion testing if pass no output
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
