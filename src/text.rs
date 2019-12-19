use crate::line::Line;
use crate::line_parser::LineParser;

/// a text, that is just a collection of lines
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Text<'a> {
    pub lines: Vec<Line<'a>>,
}

impl<'s> From<&'s str> for Text<'s> {
    fn from(md: &str) -> Text<'_> {
        let mut lines = Vec::new();
        let mut between_fences = false;
        for md_line in md.lines() {
            let parser = LineParser::from(md_line);
            let line = if between_fences {
                parser.as_code()
            } else {
                parser.line()
            };
            match line {
                Line::CodeFence => {
                    between_fences = !between_fences;
                }
                _ => {
                    lines.push(line);
                }
            }
        }
        Text { lines }
    }
}
