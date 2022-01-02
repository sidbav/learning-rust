// Variables are immutable by default
// Rust is a block scoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;

    println!("My name is {} and I am {} years old", name, age);

    age = 56;

    println!("My name is {} and I am {} years old", name, age);

    // Defining constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Multi assign
    let (my_name, my_age) = ("Brad", 37);

    println!("{} is {}", my_name, my_age);
}
