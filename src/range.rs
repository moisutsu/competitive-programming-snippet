use cargo_snippet::snippet;

#[snippet("@range")]
macro_rules! range {
    ($stop:expr) => {
        0..$stop
    };
    ($start:expr, $stop:expr) => {
        $start..$stop
    };
    ($start:expr, $stop:expr, -$step:expr) => {
        ($stop + 1..$start + 1).rev().step_by($step)
    };
    ($start:expr, $stop:expr, $step:expr) => {
        ($start..$stop).step_by($step)
    };
}

#[test]
fn test_range_macro() {
    assert_eq!(range!(5), 0..5);
}
