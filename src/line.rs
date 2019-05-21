use crate::compound::Compound;
use crate::line_parser::LineParser;

pub const MAX_HEADER_DEPTH: usize = 8;

/// The global style of a line
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineStyle {
    Normal,
    Header(u8), // never 0, and <= MAX_HEADER_DEPTH
    ListItem,
    Code,
}

/// a parsed line
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
    pub fn is_code(&self) -> bool {
        match self.style {
            LineStyle::Code => true,
            _ => false,
        }
    }
    pub fn is_list_item(&self) -> bool {
        match self.style {
            LineStyle::ListItem => true,
            _ => false,
        }
    }
    // return the total number of characters in the parsed line
    // Example
    // ```rust
    // assert_eq!(Line::from("τ:`2π`").char_length(), 4);
    // ```
    pub fn char_length(&self) -> usize {
        self.compounds
            .iter()
            .fold(0, |sum, compound| sum + compound.as_str().chars().count())
    }
}

#[test]
pub fn count_chars() {
    assert_eq!(Line::from("τ").char_length(), 1);
    assert_eq!(Line::from("τ:`2π`").char_length(), 4);
    assert_eq!(Line::from("* item").char_length(), 4);
}
