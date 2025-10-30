use std::io;

fn main() {
    
    println!("Guess the number!");

    println!("Please input your guess: ");

    let apple = 5; //immutable - CONSTANT
    let mut guess = String::new(); //mutable - VARIABLE

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");

    println!("You guessed: {guess}");

}