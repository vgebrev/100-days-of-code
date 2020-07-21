use day_016_testing;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, day_016_testing::adder::add_two(2));
}