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
    /// Parse a text from markdown lines.
    ///
    /// To build a text with parsing options, prefer the
    /// `termimad::parse_text` function
    pub fn from_md_lines<I>(md_lines: I) -> Self
    where
        I: Iterator<Item = &'s str>,
    {
        crate::parser::parse_lines(md_lines, Options::default())
    }
}
