use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the random number is {}",secret_number);

    loop{    
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit"{
            break;
        }
        let guess: u32 = 
            match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {println!("please enter a number!"); continue; }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small!".truecolor(255, 0, 0)),
            Ordering::Greater => println!("{}","Too Big!".truecolor(255, 0, 0)),
            Ordering::Equal => {
                println!("{}","Thats Right!!!!".truecolor(0, 255, 0).bold());
                break;
            }
        }
    }
}
