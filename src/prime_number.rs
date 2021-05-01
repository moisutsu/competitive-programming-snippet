use cargo_snippet::snippet;

use std::ops::*;

#[snippet("@is_prime", prefix = "use std::ops::*;")]
fn is_prime<T>(x: T) -> bool
where
    T: Default
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + std::convert::From<bool>
        + AddAssign
        + PartialEq
        + PartialOrd
        + Copy,
{
    let zero = T::default();
    if x == zero {
        return false;
    }
    let one = true.into();
    if x == one {
        return false;
    }
    let mut i = one + one;
    while i * i <= x {
        if x % i == zero {
            return false;
        }
        i += one;
    }
    true
}

#[snippet("@eratosthenes")]
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

#[snippet("@prime_factorization")]
fn prime_factorization(n: u128) -> std::collections::HashMap<u128, u128> {
    let mut map = std::collections::HashMap::new();
    let mut n = n;
    if n == 1 {
        map.insert(n, 1);
        return map;
    }
    while n % 2 == 0 {
        n /= 2;
        *map.entry(2).or_insert(0) += 1;
    }
    let mut x = 3;
    while x * x <= n {
        if n % x == 0 {
            *map.entry(x).or_insert(0) += 1;
            n /= x;
        } else {
            x += 2;
        }
    }
    if n > 1 {
        *map.entry(n).or_insert(0) += 1;
    }
    map
}

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(0), false);
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2usize), true);
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

#[test]
fn test_prime_factorization() {
    use maplit::hashmap;
    assert_eq!(
        prime_factorization(1),
        hashmap! {
            1 => 1,
        }
    );
    assert_eq!(
        prime_factorization(2u128),
        hashmap! {
            2u128 => 1,
        }
    );
    assert_eq!(
        prime_factorization(20),
        hashmap! {
            2 => 2,
            5 => 1,
        }
    );
    assert_eq!(
        prime_factorization(97),
        hashmap! {
            97 => 1,
        }
    );
    assert_eq!(
        prime_factorization(121),
        hashmap! {
            11 => 2,
        }
    );
    assert_eq!(
        prime_factorization(121),
        hashmap! {
            11 => 2,
        }
    );
    assert_eq!(
        prime_factorization(3672),
        hashmap! {
            2 => 3,
            3 => 3,
            17 => 1,
        }
    );
}
