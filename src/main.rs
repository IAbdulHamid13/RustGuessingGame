use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=10);
    // println!("Guess the secret number {secret_number}");
    loop {
        println!("Enter your guess: ");
        let mut guess = String::new(); //Need this to read number on input
        io::stdin().read_line(&mut guess).expect("Failed  to read line.");
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("Your guess was {guess}");
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small"),
            Ordering::Equal =>
                {
                    println!("Just right");
                    break;
                }
            Ordering::Greater => println!("Too big"),
        }
    }
}