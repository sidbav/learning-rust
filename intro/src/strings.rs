/* Two different types of strings
 * Primitive String = Immutable fixed length string somewhere in memeory
 * String = Growable, heap-allocated data structure - use when you need to modify of own string
 * data. 
 */

// There are a lot of string methods, read documentation

pub fn run() {
    // let hello = "Hello"; // immutable
    let mut hello = String::from("Hello ");

    println!("Before: {}", hello);
    println!("Length {}", hello.len());

    // Pushing a Char
    hello.push('W');
    println!("After: {}", hello);

    // Pushing a string
    hello.push_str("orld");
    println!("After: {}", hello);

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains \"World\", {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    println!();
    // Create a string with a specific capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
