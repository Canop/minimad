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
                Line::new_code(Compound::raw_str("a")),
                Line::new_code(Compound::raw_str("    b")),
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

// CommonMark list nesting tests
#[test]
fn commonmark_basic_list_nesting() {
    use crate::*;
    let md = r#"* level 0
  * level 1
   * level 2"#;
    let text = parse_text(md, Options::default());
    assert_eq!(text.lines.len(), 3);

    if let Line::Normal(composite) = &text.lines[0] {
        assert_eq!(composite.style, CompositeStyle::ListItem(0));
    } else {
        panic!("First line should be a list item");
    }

    if let Line::Normal(composite) = &text.lines[1] {
        assert_eq!(composite.style, CompositeStyle::ListItem(1));
    } else {
        panic!("Second line should be a list item");
    }

    if let Line::Normal(composite) = &text.lines[2] {
        assert_eq!(composite.style, CompositeStyle::ListItem(1));
    } else {
        panic!("Third line should be a list item");
    }
}

#[test]
fn commonmark_list_nesting_with_different_indents() {
    use crate::*;
    let md = r#"* level 0
 * not nested (1 space)
  * nested (2 spaces)
   * nested (3 spaces)"#;
    let text = parse_text(md, Options::default());
    assert_eq!(text.lines.len(), 4);

    if let Line::Normal(composite) = &text.lines[1] {
        assert_eq!(
            composite.style,
            CompositeStyle::ListItem(0),
            "Line 1 with 1 space should NOT be nested"
        );
    }

    if let Line::Normal(composite) = &text.lines[2] {
        assert_eq!(
            composite.style,
            CompositeStyle::ListItem(1),
            "Line 2 with 2 spaces should be nested"
        );
    }

    if let Line::Normal(composite) = &text.lines[3] {
        assert_eq!(
            composite.style,
            CompositeStyle::ListItem(1),
            "Line 3 with 3 spaces should be nested"
        );
    }
}

#[test]
fn commonmark_list_nesting_with_4_spaces() {
    use crate::*;
    // 4 spaces with list marker creates nested list item, not code block
    // This allows deeper nesting beyond 3 levels
    let md = r#"* level 0
    * level 1"#;
    let text = parse_text(md, Options::default());
    assert_eq!(text.lines.len(), 2);

    if let Line::Normal(composite) = &text.lines[1] {
        assert_eq!(composite.style, CompositeStyle::ListItem(1));
    } else {
        panic!("Second line should be nested list item");
    }
}

#[test]
fn commonmark_ordered_list_nesting() {
    use crate::*;
    let md = r#"1. level 0
   2. level 1"#;
    let text = parse_text(md, Options::default());
    assert_eq!(text.lines.len(), 2);

    if let Line::Normal(composite) = &text.lines[1] {
        assert_eq!(composite.style, CompositeStyle::ListItem(1));
    }
}

#[test]
fn commonmark_list_same_level() {
    use crate::*;
    let md = r#"* item 1
* item 2
* item 3"#;
    let text = parse_text(md, Options::default());
    assert_eq!(text.lines.len(), 3);

    for (i, line) in text.lines.iter().enumerate() {
        if let Line::Normal(composite) = line {
            assert_eq!(
                composite.style,
                CompositeStyle::ListItem(0),
                "Line {} should be at depth 0",
                i
            );
        } else {
            panic!("Line {} should be a list item", i);
        }
    }
}

#[test]
fn commonmark_list_mixed_markers() {
    use crate::*;
    let md = r#"- parent
  * child"#;
    let text = parse_text(md, Options::default());
    assert_eq!(text.lines.len(), 2);

    if let Line::Normal(composite) = &text.lines[1] {
        assert_eq!(composite.style, CompositeStyle::ListItem(1));
    }
}

#[test]
fn commonmark_tab_indented_list() {
    use crate::*;
    let md = "- parent\n\t- child\n- sibling";
    let text = parse_text(md, Options::default());
    assert_eq!(text.lines.len(), 3);

    if let Line::Normal(composite) = &text.lines[1] {
        assert_eq!(
            composite.style,
            CompositeStyle::ListItem(1),
            "Tab-indented line should be nested"
        );
    }
}

#[test]
fn tab_indented_children_after_shifted_top_level_item() {
    use crate::*;
    let md = "\t- shifted top level\n- parent\n\t- child";
    let text = parse_text(md, Options::default());
    assert_eq!(text.lines.len(), 3);

    if let Line::Normal(composite) = &text.lines[2] {
        assert_eq!(
            composite.style,
            CompositeStyle::ListItem(1),
            "Tab-indented child should nest under the latest parent context"
        );
    } else {
        panic!("Third line should be a nested list item");
    }
}
