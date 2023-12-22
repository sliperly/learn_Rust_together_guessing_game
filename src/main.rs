use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number: u8 = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        println!("You guessed: {}", guess);
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number from 1 to 101!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Equal => {
                println!("You win.");
                break;
            },
            Ordering::Greater => println!("Too big."),
        }
    }
}

