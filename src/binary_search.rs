use cargo_snippet::snippet;

#[snippet("@binary_search")]
pub trait BinarySearch {
    type Element;
    fn lower_bound(&self, x: &Self::Element) -> usize;
    fn upper_bound(&self, x: &Self::Element) -> usize;
}

#[snippet("@binary_search")]
impl<T: Ord> BinarySearch for [T] {
    type Element = T;
    fn lower_bound(&self, x: &Self::Element) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(&x) {
                std::cmp::Ordering::Less => {
                    low = mid + 1;
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &Self::Element) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(&x) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    low = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

#[test]
fn test_binary_search() {
    let vec = vec![1, 2, 4, 4, 6, 7, 12, 54, 60];
    // lower_bound
    assert_eq!(vec.lower_bound(&0), 0);
    assert_eq!(vec.lower_bound(&1), 0);
    assert_eq!(vec.lower_bound(&4), 2);
    assert_eq!(vec.lower_bound(&5), 4);
    assert_eq!(vec.lower_bound(&60), 8);
    assert_eq!(vec.lower_bound(&100), 9);

    // upper_bound
    assert_eq!(vec.upper_bound(&0), 0);
    assert_eq!(vec.upper_bound(&1), 1);
    assert_eq!(vec.upper_bound(&4), 4);
    assert_eq!(vec.upper_bound(&5), 4);
    assert_eq!(vec.upper_bound(&60), 9);
    assert_eq!(vec.upper_bound(&100), 9);
}
