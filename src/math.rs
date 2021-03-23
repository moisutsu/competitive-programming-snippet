use cargo_snippet::snippet;

use std::ops::*;

#[snippet(name = "@gcd", prefix = "use std::ops::*;")]
fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Default + Ord + Sub<Output = T> + Rem<Output = T>,
{
    let (mut a, mut b) = if a < b { (b, a) } else { (a, b) };
    let zero = T::default();
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
    T: Copy + Default + Ord + Sub<Output = T> + Rem<Output = T> + Mul<Output = T> + Div<Output = T>,
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

#[snippet("@mod_pow")]
macro_rules! mod_pow {
    ($a:expr, $n:expr, $mod:expr) => {{
        let mut ret = 1;
        let mut a = $a;
        let mut n = $n;
        while n > 0 {
            if n & 1 == 1 {
                ret = ret * a % $mod;
            }
            a = a * a % $mod;
            n >>= 1;
        }
        ret
    }};
}

#[snippet("@mod_inv")]
macro_rules! mod_inv {
    ($a:expr, $mod:expr) => {{
        let (mut a, mut b, mut u, mut v) = ($a, $mod, 1, 0);
        while b != 0 {
            let t = a / b;
            a -= t * b;
            u -= t * v;
            b = std::mem::replace(&mut a, b);
            v = std::mem::replace(&mut u, v);
        }
        u %= $mod;
        if u < 0 {
            u += $mod;
        }
        u
    }};
}

#[snippet("@mod_comb")]
struct ModComb {
    fac: Vec<usize>,
    finv: Vec<usize>,
    modulo: usize,
}

#[snippet("@mod_comb")]
impl ModComb {
    fn new(n: usize, modulo: usize) -> Self {
        let mut fac = vec![1; n + 1];
        let mut finv = vec![1; n + 1];
        let mut inv = vec![1; n + 1];
        for i in 2..=n {
            fac[i] = fac[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            finv[i] = finv[i - 1] * inv[i] % modulo;
        }
        ModComb { fac, finv, modulo }
    }

    fn comb(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        self.fac[n] * (self.finv[k] * self.finv[n - k] % self.modulo) % self.modulo
    }
}

#[snippet("@to_base")]
trait ToBase<T> {
    fn to_base(&self, base: T) -> T;
}

#[snippet("@to_base")]
impl<T> ToBase<T> for T
where
    T: Copy
        + Default
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::cmp::PartialOrd,
{
    fn to_base(&self, base: T) -> T {
        let zero = T::default();
        let one = base / base;
        let ten = (one + one + one) * (one + one + one) + one;

        let mut n = *self;
        let mut ret = zero;
        let mut digit = one;

        while n > zero {
            ret = ret + n % base * digit;
            n = n / base;
            digit = digit * ten;
        }

        ret
    }
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

#[test]
fn test_mod_pow() {
    assert_eq!(mod_pow!(3, 45, 1000000007), 644897553i64);
    assert_eq!(mod_pow!(5, 50, 1000000007), 876125953i128);
    assert_eq!(mod_pow!(8, 100, 1000000007), 322050759usize);
}

#[test]
fn test_mod_inv() {
    const MOD: i64 = 1000000007;
    let mut a = 12345678900000;
    let b = 100000;
    a %= MOD;
    assert_eq!(a * mod_inv!(b, MOD) % MOD, 123456789);
}

#[test]
fn test_to_base() {
    let n = 100i128;
    assert_eq!(n.to_base(2), 1100100);
    assert_eq!(n.to_base(8), 144);
    assert_eq!(n.to_base(16), 64);

    let n = 492usize;
    assert_eq!(n.to_base(2), 111101100);
    assert_eq!(n.to_base(8), 754);
}
