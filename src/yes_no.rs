use cargo_snippet::snippet;

#[snippet("@YesNo")]
fn YesNo(flag: bool) -> &'static str {
    if flag {
        "Yes"
    } else {
        "No"
    }
}

#[snippet("@YESNO")]
fn YESNO(flag: bool) -> &'static str {
    if flag {
        "YES"
    } else {
        "NO"
    }
}

#[test]
fn test_yesno() {
    assert_eq!(YESNO(true), "YES");
    assert_eq!(YESNO(false), "NO");
    assert_eq!(YesNo(true), "Yes");
    assert_eq!(YesNo(false), "No");
}
