use quork_proc::{lstrip_lines, rstrip_lines, strip_lines};


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

#[test]
fn test_multiline_left() {
    let expected = "function foo() {
let foo = \"bar\";
return foo;
}
";

    let actual = lstrip_lines!(
        "
        function foo() {
            let foo = \"bar\";
            return foo;
        }
    "
    );

    assert_eq!(actual, expected);
}

#[test]
fn test_multiline_right() {
    let expected = "        function foo() {
            return \"bar\";
        }
    ";

    #[rustfmt::skip]
    let actual = rstrip_lines!(
        "
        function foo() {
            return \"bar\";
        }\t\t\t
    "
    );

    assert_eq!(actual, expected);
}
