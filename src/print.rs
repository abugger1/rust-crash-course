pub fn run() {
    //print to console
    println!("Hello from the print.rs file :D");

    // Basic Formatting
    println!("Number: {}", 1);

    println!("{} is my {}", "Bruno", "Cane Corso");

    // Postitional Arguments
    println!("{0} is my {1} and {0} likes to {2}", "Bruno", "Cane Corso", "play");

    // Named Arguments
    println!("{name} likes to {activity}", name = "Bruno", activity = "Tug");

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "String"));
}
