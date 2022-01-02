// Many different types of loops in rust

pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 20 {break;};
    }

    // While Loop

    count = 0;
    while count <= 100 {
        print!("{} ", count);
        if count % 3 == 0 {
            print!("Fizz");
        }
        if count % 5 == 0 {
            print!("Buzz");
        }
        print!("\n");

        count += 1;
    }

    // for range loop
    for x in 0..100 {
        print!("{} ", x);
        if x % 3 == 0 {
            print!("Fizz");
        }
        if x % 5 == 0 {
            print!("Buzz");
        }
        print!("\n");
    }

}
