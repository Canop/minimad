
use crate::compound::{Compound};
use crate::parser::LineParser;

#[derive(Debug, PartialEq, Eq)]
pub struct Line<'a> {
    pub compounds: Vec<Compound<'a>>,
}

impl Line<'_> {
    pub fn new<'a>() -> Line<'a> {
        Line {
            compounds: Vec::new(),
        }
    }
    pub fn from(md: &str) -> Line {
        LineParser::from(md).line()
    }
}
