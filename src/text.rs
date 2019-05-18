use crate::line::Line;
use crate::line_parser::LineParser;

/// a text, that is just a collection of lines
#[derive(Debug, PartialEq, Eq)]
pub struct Text<'a> {
    pub lines: Vec<Line<'a>>,
}

impl Text<'_> {
    /// build a parsed text from a raw one
    pub fn from(md: &str) -> Text {
        let lines = md.lines().map(|md| LineParser::from(md).line()).collect();
        Text { lines }
    }
}
