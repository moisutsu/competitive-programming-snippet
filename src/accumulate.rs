use cargo_snippet::snippet;

#[snippet("@accumulate")]
macro_rules! accumulate {
    ($v:expr) => {{
        let mut accumulate = vec![0];
        accumulate.extend(
            $v.iter()
                .scan(0, |state, &x| {
                    *state += x;
                    Some(*state)
                })
                .collect::<Vec<_>>(),
        );
        accumulate
    }};
}

#[test]
fn test_accumulate() {
    let xs = vec![2, 4, 7, 10];
    assert_eq!(accumulate!(xs), vec![0, 2, 6, 13, 23]);
}
