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

fn eratosthenes(upper_limit: usize) -> Vec<usize> {
    if upper_limit == 0 {
        return vec![];
    }
    let mut primes = vec![];
    let mut is_primes = vec![true; upper_limit + 1];
    is_primes[0] = false;
    is_primes[1] = false;
    for p_i in 2..=upper_limit {
        if !is_primes[p_i] {
            continue;
        }
        primes.push(p_i);
        for m_i in (p_i * p_i..upper_limit + 1).step_by(p_i) {
            is_primes[m_i] = false;
        }
    }
    primes
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

#[test]
fn test_eratosthenes() {
    assert_eq!(eratosthenes(0), vec![]);
    assert_eq!(eratosthenes(1), vec![]);
    assert_eq!(eratosthenes(5), vec![2, 3, 5]);
    assert_eq!(eratosthenes(11), vec![2, 3, 5, 7, 11]);
    assert_eq!(
        eratosthenes(50),
        vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
    );
}
