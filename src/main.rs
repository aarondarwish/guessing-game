use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Ask the user to guess a number. 
    println!("Guess the number!\n Please input your guess.");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}\nPlease input your guess.");

    
    loop {
        // Collect the answer from the user.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
    
        // Shadow the guess variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
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
