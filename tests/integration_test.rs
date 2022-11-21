use aolifu_rust::function;

mod common;
#[test]
fn cal_largest_test() {
    common::setup();
    function::cal_largest_two();
}