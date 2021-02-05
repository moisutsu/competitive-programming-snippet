use cargo_snippet::snippet;

#[snippet("@get_index")]
trait GetIndex {
    type Element;
    fn get_index(&self, value: &Self::Element) -> Option<usize>;
}

#[snippet("@get_index")]
impl<T: PartialEq> GetIndex for [T] {
    type Element = T;
    fn get_index(&self, value: &Self::Element) -> Option<usize> {
        self.iter().position(|x| x == value)
    }
}

#[test]
fn test_index() {
    let num_v = vec![0, 3, 5, 10];
    assert_eq!(num_v.get_index(&0), Some(0));
    assert_eq!(num_v.get_index(&3), Some(1));
    assert_eq!(num_v.get_index(&10), Some(3));
    assert_eq!(num_v.get_index(&7), None);

    let str_v = vec!["Hello", "World"];
    assert_eq!(str_v.get_index(&"Hello"), Some(0));
    assert_eq!(str_v.get_index(&"Hello"), Some(0));
    assert_eq!(str_v.get_index(&"ABC"), None);
}
