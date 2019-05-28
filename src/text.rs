use crate::line::Line;
use crate::line_parser::LineParser;

/// a text, that is just a collection of lines
#[derive(Debug, PartialEq, Eq)]
pub struct Text<'a> {
    pub lines: Vec<Line<'a>>,
}

impl Text<'_> {
    /// build a parsed text from a Markdown string
    pub fn from(md: &str) -> Text<'_> {
        let lines = md.lines().map(|md| LineParser::from(md).line()).collect();
        Text { lines }
    }
}
