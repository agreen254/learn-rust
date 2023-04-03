use rand::Rng; // Rng is a trait, meaning that it defines methods that the random number generators implement
use std::cmp::Ordering;
use std::io;

fn main() {
    // range expression, start..=end
    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secret_num}");

    loop {
        println!("Guess the number!");

        println!("input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line.");

        // use shadowing to convert guess from a String to a u32
        // also we must trim, because a newline character '\n' is automatically
        // appended whenever the user hits the enter key
        // so the raw input would be '58\n'
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.\n");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
