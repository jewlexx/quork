// The struct instantiation has "too many arguments"
#![allow(clippy::too_many_arguments)]

use quork_proc::*;

#[derive(Debug, FromTuple, PartialEq, Eq)]
struct TestStruct<'a, T>
where
    T: std::fmt::Debug,
{
    pub a: i32,
    pub(crate) b: i32,
    c: usize,
    some_super_long_identifier_for_a_string: String,
    lifetimes: &'static str,
    custom_lifetime: &'a str,
    generic: T,
    combined: &'a T,
}

#[derive(Debug, FromTuple, PartialEq, Eq)]
struct TestTupleStruct<'a, T>(i32, i32, usize, String, &'static str, &'a str, T, &'a T)
where
    T: std::fmt::Debug;

#[test]
fn test_simple_struct() {
    let tuple = (1, 2, 3, "Foo".to_string(), "Bar", "Baz", "could", &"be");

    let test_struct = TestTupleStruct::from(tuple);

    let expected = TestTupleStruct(1, 2, 3, "Foo".to_string(), "Bar", "Baz", "could", &"be");

    assert_eq!(dbg!(test_struct), expected);
}

#[test]
fn test_complex_struct() {
    let tuple = (
        1,
        2,
        3,
        "Foo".to_string(),
        "Bar",
        "Baz",
        "could be anything really",
        &"still could be really anything",
    );

    let test_struct: TestStruct<&str> = tuple.into();

    let expected = TestStruct {
        a: 1,
        b: 2,
        c: 3,
        some_super_long_identifier_for_a_string: "Foo".to_string(),
        lifetimes: "Bar",
        custom_lifetime: "Baz",
        generic: "could be anything really",
        combined: &"still could be really anything",
    };

    assert_eq!(dbg!(test_struct), expected);
}
