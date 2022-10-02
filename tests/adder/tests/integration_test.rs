use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    let result = adder::add_two(2);
    assert_eq!(4, result);
}
