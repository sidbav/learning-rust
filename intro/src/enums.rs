
enum Movement {
    Up,
    Down,
    Left,
    Right
}

pub fn run() {
    let av1 = Movement::Left;
    let av2 = Movement::Up;
    let av3 = Movement::Right;
    let av4 = Movement::Down;

    move_avatar(av1);
    move_avatar(av2);
    move_avatar(av3);
    move_avatar(av4);
}

// Perform an action depending on arg
fn move_avatar(m:Movement) {
    match m {
        Movement::Up => println!("Moving up"),
        Movement::Down => println!("Moving down"),
        Movement::Left => println!("Moving left"),
        Movement::Right => println!("Moving right")
    }
}
