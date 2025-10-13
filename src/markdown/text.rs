use crate::*;

/// a text, that is just a collection of lines
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Text<'a> {
    pub lines: Vec<Line<'a>>,
}

impl<'s> From<&'s str> for Text<'s> {
    /// Build a text from a multi-line string interpreted as markdown
    ///
    /// To build a text with parsing options, prefer the
    /// `termimad::parse_text` function
    fn from(md: &str) -> Text<'_> {
        parse_text(md, Options::default())
    }
}

impl<'s> Text<'s> {
    pub fn from_str(
        s: &'s str,
        options: Options,
    ) -> Self {
        crate::parser::parse_lines(s.lines(), options)
    }
    /// Parse a text from markdown lines.
    pub fn from_md_lines<I>(md_lines: I) -> Self
    where
        I: Iterator<Item = &'s str>,
    {
        crate::parser::parse_lines(md_lines, Options::default())
    }
    pub fn raw_str(s: &'s str) -> Self {
        let lines = s.lines().map(Line::raw_str).collect();
        Self { lines }
    }
}

#[test]
fn test_keep_code_fences() {
    let md = r"# Heading
Simple text.
```rust
    let a = 10;
    let b = 20;
```
";

    // not keeping code fences
    let text = Text::from_str(md, Options::default().keep_code_fences(false));
    assert_eq!(text.lines.len(), 4);

    // keeping code fences
    let text = Text::from_str(md, Options::default().keep_code_fences(true));
    dbg!(&text);
    assert_eq!(text.lines.len(), 6);
    let lang = text.lines[2].code_fence_lang().unwrap();
    assert_eq!(lang, "rust");
}
