use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Ask the user to guess a number. 
    println!("A secret number between 1 and 10 has been generated. Try to guess the number!\n Please input your guess.");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);

    loop {
        // Collect the answer from the user.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
    
        // Shadow the guess variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("What you entered is an invalid. Please enter a integer.");
                continue;
            }
        };
    
        // Inform the user about the guess value.
        println!("You guessed: {}", guess);
    
            match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small."),
                    Ordering::Greater => println!("Too big."),
                    Ordering::Equal => {
                        println!("You win.");
                        break;
                    }
                }
        
            }
}
