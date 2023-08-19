#[cfg(test)]
mod tests {
    use std::io;
    use crate::games::guessing::check_guess;


    #[test]
    fn test_correct_guess(){
        let secret = 43;
        let guess = Ok("43".to_string());
        check_guess(secret, guess)
    }

    #[test]
    fn test_incorrect_guess(){
        let secret = 45;
        let guess = Ok("43".to_string());
        check_guess(secret, guess)
    }

    #[test]
    fn test_invalid_guess() {
        let secret = 42;
        let guess = Ok("not a number".to_string());
        check_guess(secret, guess);
    }

    #[test]
    fn test_read_error() {
        let secret = 42;
        let guess = Err(io::Error::new(io::ErrorKind::Other, "read error"));
        check_guess(secret, guess);
    }

}