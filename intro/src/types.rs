// Type does not need to be set explictlly, compiler will usually guess what
// the type


pub fn run() {
    // Default is i32
    let x = 4;

    // Default is f64
    let y = 4.3;

    // Find max size of a data type
    println!("Max i32:{}", std::i32::MAX);
    println!("Max i64:{}", std::i64::MAX);


    // Boolean values
    let is_active: bool = true;

    // Get Boolean from expression
    let is_greater: bool = 10 > 5;

    let a1 = '1';

    let face = '\u{1F600}';

    println!("{:?}", (x, y, is_active, is_greater, a1, face));

}

