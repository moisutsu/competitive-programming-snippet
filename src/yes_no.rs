use cargo_snippet::snippet;

#[snippet("@YesNo")]
macro_rules! YesNo {
    ($flag:expr) => {
        if $flag {
            "Yes"
        } else {
            "No"
        }
    };
}

#[snippet("@YESNO")]
macro_rules! YESNO {
    ($flag:expr) => {
        if $flag {
            "YES"
        } else {
            "NO"
        }
    };
}

#[test]
fn test_yesno() {
    assert_eq!(YESNO!(true), "YES");
    assert_eq!(YESNO!(false), "NO");
    assert_eq!(YesNo!(true), "Yes");
    assert_eq!(YesNo!(false), "No");
}
