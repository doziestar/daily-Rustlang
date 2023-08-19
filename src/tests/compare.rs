#[cfg(test)]
mod tests {
    use crate::games::guessing::check_guess_comprisim;

    #[test]
    fn test_less_than(){
        let secret = "45".to_string();
        let guess = Ok("43".to_string());
        let result = check_guess_comprisim(secret, guess).unwrap();
        assert_eq!(result, 1)
    }

    #[test]
    fn test_greater_than(){
        let secret = "45".to_string();
        let guess = Ok("48".to_string());
        let result = check_guess_comprisim(secret, guess).unwrap();
        assert_eq!(result, 2)
    }

    #[test]
    fn test_equal_than(){
        let secret = "45".to_string();
        let guess = Ok("45".to_string());
        let result = check_guess_comprisim(secret, guess).unwrap();
        assert_eq!(result, 3)
    }
}