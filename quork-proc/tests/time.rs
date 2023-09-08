use quork_proc::time;

#[test]
#[time]
fn test_time_maths() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[time(nanoseconds)]
fn test_time_nanoseconds() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_time_unsafe() {
    #[time]
    unsafe fn unsafe_fn() {
        assert_eq!(2 + 2, 4);
    }

    unsafe { unsafe_fn() };
}
