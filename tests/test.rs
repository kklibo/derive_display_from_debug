
use std::fmt::Display;

use derive_display_from_debug::Display;

#[derive(Debug, Display)]
struct TestStruct {}

#[test]
fn test() {

    assert_eq!("TestStruct", format!("{:?}", TestStruct{}));
    assert_eq!("TestStruct", format!("{}", TestStruct{}));
}
