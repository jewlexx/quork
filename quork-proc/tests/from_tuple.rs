// The struct instantiation has "too many arguments"
#![allow(clippy::too_many_arguments)]

use quork_proc::*;

#[derive(Debug, FromTuple)]
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

#[derive(Debug, FromTuple)]
struct TestTupleStruct<'a, T>(i32, i32, usize, String, &'a str, T, &'a T)
where
    T: std::fmt::Debug;
