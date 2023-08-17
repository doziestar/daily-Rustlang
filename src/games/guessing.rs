use std::io;

pub fn check_test(a: usize, b:usize) -> usize{
    a + b
}

pub fn read_from_terminal() -> Result<String, io::Error> {
    println!("Guess a number!");
    println!("Please Input your guess.");
    let mut guess = String::new();

    io::stdin().
        read_line(&mut guess)?;

    println!("Your guess is very great");

    Ok(guess.trim().to_string())
}