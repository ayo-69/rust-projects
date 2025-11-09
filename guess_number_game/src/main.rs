use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut guess: i32;
    let mut computer_guess: i32;

    let mut input = String::new();
    println!("---------------------------------------------");
    println!("---    Guess The Number Game              ---");
    println!("---------------------------------------------");

    loop {
        println!("");
        print!("Enter a number between 1 - 10, or q to quit: ");

        let mut input = String::new();

        // Get user input
        println!("");
        print!("Enter a number between 1 - 10, or q to quit: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        // Handle quit logic
        if input.trim() == "q" {
            println!();
            break;
        } else {
            println!("enter a valid option");
        }

        guess = input.trim().parse().expect("failed to parse input");
        computer_guess = rng.gen_range(1..=10);

        match guess.cmp(&computer_guess) {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("You win");
                computer_guess = rng.gen_range(1..=10);
            }
            Ordering::Greater => println!("Too high"),
        }
    }
}
