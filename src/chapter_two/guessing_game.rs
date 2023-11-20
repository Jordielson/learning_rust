use std::{io, cmp::Ordering};
use rand::{thread_rng, Rng};

pub fn processing_guess() -> () {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
    
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guess: {guess}");
    
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