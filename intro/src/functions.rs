// functions - store blocks for code for reuse
//
//

pub fn run() {
    greeting("Hello", "Sid");

    println!("{}", add(1,43));

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1:i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum {}", add_nums(3, 6554));
}

fn greeting(greet: &str, name:&str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1:i32, n2:i32) -> i32 {
    n1+n2
}
