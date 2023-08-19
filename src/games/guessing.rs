use std::cmp::Ordering;
use std::io;

pub fn read_from_terminal() -> Result<String, io::Error> {
    println!("Guess a number!");
    println!("Please Input your guess.");
    let mut guess = String::new();

    io::stdin().
        read_line(&mut guess)?;

    Ok(guess.trim().to_string())
}

pub fn check_guess(secret_number: i64, guess: Result<String, io::Error>) {
    match guess {
        Ok(guess) => {
            if let Ok(parsed_guess) = guess.parse::<i64>(){
                if parsed_guess == secret_number {
                    println!("You are a genius");
                    return;
                }
            }
            println!("You got the wrong guess!")
        }
        Err(_) => println!("Failed to read your guess!")
    }
}

pub fn check_guess_comprisim(secret_number: String, guess: Result<String, io::Error>) -> Result<i32, io::Error> {
    match guess.unwrap().cmp(&secret_number) {
        Ordering::Less => Ok(1),
        Ordering::Greater => Ok(2),
        Ordering::Equal => Ok(3)
    }
}