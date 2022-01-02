pub fn run() {
    let age = 18;
    let check_id = false;
    let knows_person_of_age = true;

    // if else statement
    if knows_person_of_age || age >= 21 && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: You are underage");
    } else {
        println!("Bartender: I will need to see your IDs");
    }

    // Short hand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("is_of_age: {}", is_of_age);
}
