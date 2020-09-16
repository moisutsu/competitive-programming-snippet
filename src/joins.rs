use cargo_snippet::snippet;

#[snippet("@joins")]
trait Joins {
    fn joins(&self, sep: &str) -> String;
}

#[snippet("@joins")]
impl<T: std::string::ToString + Copy> Joins for [T] {
    fn joins(&self, sep: &str) -> String {
        self.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
}

#[test]
fn test_joins() {
    assert_eq!(vec![3, 1, 8, 5].joins(" "), "3 1 8 5");
    assert_eq!([3.0, 1.2, 8.21, 5.111].joins(", "), "3, 1.2, 8.21, 5.111");
    assert_eq!(&['a', 'c'].joins("b"), "abc");
}
