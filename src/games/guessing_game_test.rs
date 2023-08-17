use std::io;
#[cfg(test)]
use crate::games::guessing::{check_test};

#[test]
fn test_check_test() {
    let result = check_test(3, 5);
    assert_eq!(result, 8)
}

trait ReadLine {
    fn read_line(&mut self, buffer: &mut String) -> Result<(), io::Error>;
}

struct RealIo;

impl ReadLine for RealIo {
    fn read_line(&mut self, buffer: &mut String) -> Result<(), io::Error> {
        io::stdin().read_line(buffer).map(|_| ())
    }
}

fn read_from_terminal<T: ReadLine>(io: &mut T) -> Result<String, io::Error> {
    println!("Guess a number!");
    println!("Please Input your guess.");
    let mut guess = String::new();
    io.read_line(&mut guess)?;
    println!("Your guess is very great");
    Ok(guess.trim().to_string())
}

struct MockIo {
    input: String,
}

impl ReadLine for MockIo {
    fn read_line(&mut self, buffer: &mut String) -> Result<(), io::Error> {
        buffer.push_str(&self.input);
        Ok(())
    }
}

#[test]
fn test_system_readline() {
    let mut mock_io = MockIo {
        input: "start".to_string(),
    };
    let result = read_from_terminal(&mut mock_io);
    assert_eq!(result.unwrap(), "start");
}