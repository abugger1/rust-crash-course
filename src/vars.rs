// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Bruno";
    let mut age = "10 months";

    println!("My dogs name is {0} and he  was {1} old", name, age);

    age = "11 months";

    println!("A month has passed since {} then he is now {} old.", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (d_name, d_age ) = ("Bruno", "11 months");
    println!("{} is {} old.", d_name, d_age);
}
