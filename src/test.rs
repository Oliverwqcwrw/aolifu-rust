#[test]
#[ignore]
pub fn assert_test() {
    let a = 10;
    let b = 5;
    assert!(a > b, "a less than b");

    let c = 10;
    assert_eq!(a, c, "a not equal b");

    assert_ne!(a, b);
}

#[test]
#[should_panic]
pub fn should_panic_test() {
    let a = 10;
    if a > 0 {
        panic!("should panic");
    }
}

#[test]
#[should_panic(expected = "should panic")]
pub fn should_panic_test_two() {
    let a = 10;
    if a > 0 {
        panic!("should panic");
    }
}

#[test]
fn result_test() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two doesn't equal to four"))
    }
}