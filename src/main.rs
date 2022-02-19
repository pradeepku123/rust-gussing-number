use std::io;
fn main() {
    println!("Guss the number! game");

    println!("Please input your guess.");

    //guss variable bind with new Empty String
    // ::new() associate function of String Type
    let mut guess = String::new(); // by default varivables are immutable to make variable mutable we use "mut" key word

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("Your guessed: {}", guess);
}
