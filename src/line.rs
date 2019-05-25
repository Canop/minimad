use crate::composite::{Composite, CompositeStyle};
use crate::compound::Compound;
use crate::line_parser::LineParser;
use crate::tbl::TableRow;

pub const MAX_HEADER_DEPTH: usize = 8;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CellFormat {
    alignment: Alignment,
}

/// a parsed line
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Line<'a> {
    Normal(Composite<'a>),
    TableRow(TableRow<'a>),
}

impl Line<'_> {
    pub fn from(md: &str) -> Line {
        LineParser::from(md).line()
    }
    #[inline(always)]
    pub fn char_length(&self) -> usize {
        match self {
            Line::Normal(composite) => composite.char_length(),
            Line::TableRow(row) => row.cells.iter().fold(0, |s, c| s + c.char_length()),
        }
    }
    pub fn new_paragraph(compounds: Vec<Compound>) -> Line {
        Line::Normal(Composite {
            style: CompositeStyle::Paragraph,
            compounds,
        })
    }
    pub fn new_code(compound: Compound) -> Line {
        Line::Normal(Composite {
            style: CompositeStyle::Code,
            compounds: vec![compound],
        })
    }
    pub fn new_list_item(compounds: Vec<Compound>) -> Line {
        Line::Normal(Composite {
            style: CompositeStyle::ListItem,
            compounds,
        })
    }
    pub fn new_header(level: u8, compounds: Vec<Compound>) -> Line {
        Line::Normal(Composite {
            style: CompositeStyle::Header(level),
            compounds,
        })
    }
    pub fn new_table_row(composites: Vec<Composite>) -> Line {
        Line::TableRow(TableRow { cells: composites })
    }
    #[inline(always)]
    pub fn is_table_row(&self) -> bool {
        match self {
            Line::TableRow(_) => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub fn is_code(&self) -> bool {
        match self {
            Line::Normal(composite) => composite.is_code(),
            _ => false,
        }
    }
}

#[test]
pub fn count_chars() {
    assert_eq!(Line::from("τ").char_length(), 1);
    assert_eq!(Line::from("τ:`2π`").char_length(), 4);
    assert_eq!(Line::from("* item").char_length(), 4);
}
