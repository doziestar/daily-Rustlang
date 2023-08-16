use crate::games::guessing::check_test;

#[test]
fn test_check_test(){
    let result = check_test(3, 5);
    assert_eq!(result, 8)
}