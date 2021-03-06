use cargo_snippet::snippet;

use std::ops::*;

#[snippet(name = "@gcd", prefix = "use std::ops::*;")]
fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Ord + Sub<Output = T> + Rem<Output = T>,
{
    let (mut a, mut b) = if a < b { (b, a) } else { (a, b) };
    let zero = a - a;
    while b > zero {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[snippet(name = "@lcm", include = "@gcd")]
fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + Ord + Sub<Output = T> + Rem<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    a * b / gcd(a, b)
}

#[snippet(name = "@factorial")]
fn factorial(n: usize) -> usize {
    (1..=n).product()
}

#[snippet("@divisors")]
fn divisors(n: u128) -> Vec<u128> {
    let mut small_divisors = vec![];
    let mut big_divisors = vec![];
    let mut x = 1;
    while x * x <= n {
        if n % x == 0 {
            small_divisors.push(x);
            if x != n / x {
                big_divisors.push(n / x);
            }
        }
        x += 1;
    }
    small_divisors.extend(big_divisors.into_iter().rev().collect::<Vec<u128>>());
    small_divisors
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(5i128, 5i128), 5i128);
    assert_eq!(gcd(12, 3), 3);
    assert_eq!(gcd(97usize, 17usize), 1usize);
    assert_eq!(gcd(1u128, 1u128), 1u128);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(10, 5), 10);
    assert_eq!(lcm(7usize, 11usize), 77usize);
    assert_eq!(lcm(1i128, 1i128), 1i128);
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(4), 24);
    assert_eq!(factorial(6), 720);
}

#[test]
fn test_divisors() {
    assert_eq!(divisors(1), vec![1]);
    assert_eq!(divisors(2), vec![1, 2]);
    assert_eq!(divisors(10), vec![1, 2, 5, 10]);
    assert_eq!(divisors(30), vec![1, 2, 3, 5, 6, 10, 15, 30]);
    assert_eq!(divisors(97), vec![1, 97]);
    assert_eq!(divisors(121), vec![1, 11, 121]);
}
