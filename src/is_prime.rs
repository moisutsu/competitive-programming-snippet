use cargo_snippet::snippet;

use std::ops::*;

#[snippet("@is_prime", prefix = "use std::ops::*;")]
fn is_prime<T>(x: T) -> bool
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + AddAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    let one = x / x;
    if x == one {
        return false;
    }
    let zero = x - x;
    let mut i = one + one;
    while i * i <= x {
        if x % i == zero {
            return false;
        }
        i += one;
    }
    true
}

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2usize), true);
    assert_eq!(is_prime(4f64), false);
    assert_eq!(is_prime(11i128), true);
    assert_eq!(is_prime(97u128), true);
    assert_eq!(is_prime(28107u16), false);
}
