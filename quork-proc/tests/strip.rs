use quork_proc::strip_lines;

#[test]
fn test_multiline_both() {
    let expected = "function foo() {
return \"bar\";
}
";

    let actual = strip_lines!(
        "
        function foo() {
            return \"bar\";
        }
    "
    );

    assert_eq!(actual, expected);
}
