// Vectors are Resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    // Adding on the vector
    numbers.push(6);
    numbers.push(7);

    println!("{:?}", numbers);

    // Pop of last value
    numbers.pop();

    println!("{:?}", numbers);

    //  Getting a single value
    println!("The first value of the numbers vector: {}", numbers[0]);

    // Re-assign value in array
    numbers[4] = 20;
    println!("{:?}", numbers);

    // vector length
    println!("vector Length: {}", numbers.len());

    // vector are allocated on heap
    println!("numbers vector occuplies {} bytes", std::mem::size_of_val(&numbers));
    // Getting "slices" of an array
    let slice: &[i32] = &numbers[0..5];
    println!("Slice: {:?}", slice);

    // Loop through the vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x = *x*2;
    }
    println!("{:?}", numbers);
}

