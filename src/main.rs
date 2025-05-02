use std::io;
use rand::Rng;
use std::cmp::Ordering;

// This is a simple number guessing game.
    // The program generates a random number between 1 and 100,
    // and the user has to guess it.
    // The program will give feedback on whether the guess is too high or too low.
    // The game continues until the user guesses the correct number.
    // The user can input their guess, and the program will respond with feedback.

fn main() {
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println! ("Hello there, Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        
        };

        println!("You guessed: {guess}");

            match guess.cmp (&secret_number) {
                Ordering::Less => println!("Oops, Too small!"),
                Ordering::Greater => println!(" Wow, Too big!"),
                Ordering::Equal => { 
                    println!("Congratulations, You win!");
                    break;
                }
            }

        }
    }
