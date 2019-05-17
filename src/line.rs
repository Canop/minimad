use crate::compound::Compound;
use crate::line_parser::LineParser;

/// The global style of a line
#[derive(Debug, PartialEq, Eq)]
pub enum LineStyle {
    Normal,
    Header(u8),
    ListItem,
    Code,
}

/// a parsed line:
/// - the global style of the line, if any
/// - a vector of styled parts
#[derive(Debug, PartialEq, Eq)]
pub struct Line<'a> {
    pub style: LineStyle,
    pub compounds: Vec<Compound<'a>>,
}

impl Line<'_> {
    pub fn from(md: &str) -> Line {
        LineParser::from(md).line()
    }
}
