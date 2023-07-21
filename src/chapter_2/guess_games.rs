use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn guess_games_num() {
    println!("Guess the number");
    let number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed:{}", guess);
        match guess.cmp(&number) {
            Ordering::Less => {
                println!("Too Small");
            }
            Ordering::Equal => {
                println!("You Win");
                break;
            }
            Ordering::Greater => {
                println!("Too Big");
            }
        }
    }
}
