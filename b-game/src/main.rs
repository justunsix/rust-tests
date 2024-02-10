use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    // chose a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess or q to quit:");

        // mutable variable, new empty String
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // if q, quit, otherwise continue
            Err(_) => {
                if guess.trim() == "q" {
                    break;
                } else {
                    println!("Please enter a number.");
                    continue;
                }
            }
        };

        println!("You guessed: {}", guess);

        // Check guess against secret number
        // and tell user if they are close
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
