use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {secret_number}");
    loop {
        println!("input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("Your guess is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("大了"),
            Ordering::Less => println!("小了"),
            Ordering::Equal => {
                println!("对了");
                break;
            }
        }
    }
}