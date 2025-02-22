#![allow(dead_code)]

use sunbeam_macros::ElementTypeUtils;

#[test]
fn test_get_name() {
    #[derive(ElementTypeUtils)]
    enum Test {
        Variant1,
        Variant2,
        Variant3,
    }

    assert_eq!("Variant1", Test::Variant1.get_name());
    assert_eq!("Variant2", Test::Variant2.get_name());
    assert_eq!("Variant3", Test::Variant3.get_name());
}

#[test]
fn test_get_name_with_field() {
    #[derive(ElementTypeUtils)]
    enum Test {
        Variant1(u8),
        Variant2,
        Variant3,
    }

    assert_eq!("Variant1", Test::Variant1(1).get_name());
    assert_eq!("Variant2", Test::Variant2.get_name());
    assert_eq!("Variant3", Test::Variant3.get_name());
}
