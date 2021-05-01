use cargo_snippet::snippet;

#[snippet("@sorted")]
trait Sorted {
    type Element;
    fn sorted(&self) -> Vec<Self::Element>;
    fn sorted_by_key<F: FnMut(&Self::Element) -> K, K: Ord>(&self, f: F) -> Vec<Self::Element>;
}

#[snippet("@sorted")]
impl<T> Sorted for [T]
where
    T: Clone + Ord,
{
    type Element = T;

    fn sorted(&self) -> Vec<Self::Element> {
        let mut v = <&[T]>::clone(&self).to_vec();
        v.sort();
        v
    }

    fn sorted_by_key<F: FnMut(&Self::Element) -> K, K: Ord>(&self, f: F) -> Vec<Self::Element> {
        let mut v = <&[T]>::clone(&self).to_vec();
        v.sort_by_key(f);
        v
    }
}

#[test]
fn test_sorted() {
    let v1 = vec![10, 5, -3, 2, 1, 100, -10];
    assert_eq!(v1.sorted(), vec![-10, -3, 1, 2, 5, 10, 100]);
    assert_eq!(
        v1.sorted_by_key(|&x: &i32| x.abs()),
        vec![1, 2, -3, 5, 10, -10, 100]
    );
}
