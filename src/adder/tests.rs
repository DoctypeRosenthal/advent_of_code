use super::*;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

fn it_doesnt_work() {
    assert_eq!(2 + 2, 5);
}

#[test]
fn internal() {
    assert_eq!(4, internal_adder(2, 2));
}