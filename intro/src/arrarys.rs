// Fixed Length list, elements are same data types

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    //  Getting a single value
    println!("The first value of the numbers array: {}", numbers[0]);

    // Re-assign value in array
    numbers[4] = 20;
    println!("{:?}", numbers);

    // Arrary length
    println!("Arrary Length: {}", numbers.len());

    // Arrarys are allocated on staack
    println!("numbers array occuplies {} bytes", std::mem::size_of_val(&numbers));

    // Getting "slices" of an array
    let slice: &[i32] = &numbers[0..5];
    println!("Slice: {:?}", slice);
}

