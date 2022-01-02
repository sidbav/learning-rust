pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Brad", "Utah");

    // Posistional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named Arguments
    println!("{name} likes to play {a}", name="John", a="baseball");

    // Placeholder traits
    println!("Binary: {0:b}, Hex: {0:x}, Octal: {0:o}", 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}
