mod line_parser;
mod options;
mod text_parser;

pub use {
    line_parser::*,
    options::*,
    text_parser::*,
};

#[test]
fn indented_code_between_fences() {
    use crate::*;
    let md = r#"
        outside
        ```code
        a
            b
        ```
    "#;
    assert_eq!(
        parse_text(md, Options::default().clean_indentations(true)),
        Text {
            lines: vec![
                Line::new_paragraph(vec![Compound::raw_str("outside")]),
                Line::new_code(Compound::raw_str("a").code()),
                Line::new_code(Compound::raw_str("    b").code()),
            ]
        },
    );
}

#[test]
fn test_clean() {
    use crate::*;
    let text = r#"
        bla bla bla
        * item 1
        * item 2
    "#;
    assert_eq!(
        parse_text(
            text,
            Options {
                clean_indentations: true,
                ..Default::default()
            }
        ),
        Text {
            lines: vec![
                Line::from("bla bla bla"),
                Line::from("* item 1"),
                Line::from("* item 2"),
            ]
        },
    );
}

#[test]
fn test_inline_code_continuation() {
    use crate::*;
    let md = r#"
        bla bla `code
        again` bla
    "#;
    // Without continuation
    let options = Options::default().clean_indentations(true);
    assert_eq!(
        parse_text(md, options),
        Text {
            lines: vec![Line::from("bla bla `code"), Line::from("again` bla"),]
        },
    );
    // With continuation
    let options = Options::default()
        .clean_indentations(true)
        .continue_inline_code(true);
    assert_eq!(
        parse_text(md, options),
        Text {
            lines: vec![Line::from("bla bla `code`"), Line::from("`again` bla"),]
        },
    );
}
