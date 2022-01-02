// Point to a resource in memory

pub fn run() {
    // primitive array
    let array1 = [1,2,3];
    let array2 = array1;
    println!("{:?}", array1);
    println!("{:?}", array2);
    // the values are the array when we assigned the value of array 1 to array 2

    // with non primitive, if you assign another variable to a piece of data, the first variable
    // will no longer hold that value. You will need to use a reference (&) to point to the
    // resource

    print!("\n");
    // non primitive (vector)
    let vec1: Vec<i32> = vec![4, 5, 6];
    let vec2: Vec<i32> = vec1;
    //println!("{:?}", vec1); // this will lead to an error (vec1 is "moved" to vec2)
    println!("{:?}", vec2);

    print!("\n");
    // non primitive (vector)
    let correct_vec1: Vec<i32> = vec![7, 8, 9];
    let correct_vec2: &Vec<i32> = &correct_vec1;
    println!("{:?}", correct_vec1); // this will lead to an error (vec1 is "moved" to vec2)
    println!("{:?}", correct_vec2);
}
