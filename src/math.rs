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
    (1..=n).fold(1, |sigma, i| sigma * i)
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
