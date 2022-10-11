use quork_proc::New;

#[derive(New)]
struct TestImplNew {
    pub a: i32,
    pub b: i32,
}

#[test]
fn test_new_impl() {
    let s = TestImplNew::new(1, 2);

    assert_eq!(s.a, 1);
    assert_eq!(s.b, 2);
}

const S: TestImplNew = TestImplNew::new(1, 2);

#[test]
fn test_new_const() {
    assert_eq!(S.a, 1);
    assert_eq!(S.b, 2);
}
