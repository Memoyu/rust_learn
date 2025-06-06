use adder;

mod common;

// 集成测试
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add(2, 2));
}
