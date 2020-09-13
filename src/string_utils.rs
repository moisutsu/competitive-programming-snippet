use cargo_snippet::snippet;

#[snippet("@string_to_vec_char")]
trait StringToVecChar {
    fn to_vec_char(&self) -> Vec<char>;
}

#[snippet("@string_to_vec_char")]
impl StringToVecChar for str {
    fn to_vec_char(&self) -> Vec<char> {
        self.chars().collect::<Vec<char>>()
    }
}

#[test]
fn test_string_to_vec_char() {
    assert_eq!("".to_string().to_vec_char(), vec![]);
    assert_eq!(
        "Hello".to_string().to_vec_char(),
        vec!['H', 'e', 'l', 'l', 'o']
    );
    assert_eq!("a b c".to_vec_char(), vec!['a', ' ', 'b', ' ', 'c']);
}
