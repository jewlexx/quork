use quork_proc::{lstrip_lines, rstrip_lines, strip_lines};

const WHITESPACE: &str = "           ";

fn test_multiline_both() {
    let expected = r#"function foo() {
return "bar";
}
"#;

    let actual = strip_lines!(
        r#"
        function foo() {
            return "bar";
        }
    "#
    );

    assert_eq!(actual, expected);
}

fn test_multiline_left() {
    let expected = r#"function foo() {{
let foo = "bar";
        return foo;
}}
"#;

    let actual = lstrip_lines!(
        r#"
        function foo() {
            return "bar";
        }
    "#
    );

    assert_eq!(actual, expected);
}

fn test_multiline_right() {
    let expected = r#"function foo() {{
let foo = "bar";
return foo;
}}
"#;

    let actual = rstrip_lines!(
        r#"
        function foo() {
            return "bar";
        }
    "#
    );

    assert_eq!(actual, expected);
}
