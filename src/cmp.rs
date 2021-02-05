use cargo_snippet::snippet;

#[snippet(name = "@chmin", include = "@min")]
macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

#[snippet(name = "@chmax", include = "@max")]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

#[snippet("@min")]
macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

#[snippet("@max")]
macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

#[test]
fn test_min() {
    assert_eq!(2, min!(8, 9, 23, 2, 19, 29));
    assert_eq!(0usize, min!(0usize, 8usize, 19usize));
}

#[test]
fn test_max() {
    assert_eq!(100, max!(10, 32, -32, 19, 100));
    assert_eq!(50usize, max!(20usize, 10usize, 50usize));
}

#[test]
fn test_chmin() {
    let mut x = 100;
    assert!(chmin!(x, 50));
    assert_eq!(x, 50);
    assert!(!chmin!(x, 100));
    assert_eq!(x, 50);
}

#[test]
fn test_chmax() {
    let mut x = 0;
    assert!(chmax!(x, 100));
    assert_eq!(x, 100);
    assert!(!chmax!(x, 50));
    assert_eq!(x, 100);
}
