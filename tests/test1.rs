
extern crate temp_1;
use temp_1::add_one;

#[test]
fn test(){
    assert_eq!(2,add_one(1, 1));
}

#[test]
fn test2(){
    assert_ne!(3,add_one(1, 1));
}