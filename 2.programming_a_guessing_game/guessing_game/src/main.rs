// std::io library provides you with a number of useful features, including the ability to accept user input.
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");
        
        // `mut` makes a variable mutable (In Rust, variables are immutable by default)
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // The & indicates that this argument is a reference
            .expect("Failed to read line!");
        
        // Convert string into a unsigned 32 bit integer
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Go to the next iteration of the loop
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
        }
    }
}
