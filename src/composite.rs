/// a composite is a group of compounds. It can be a whole line,
/// or a table cell
///
use crate::compound::Compound;
use crate::line_parser::LineParser;

/// The global style of a composite
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompositeStyle {
    Paragraph,
    Header(u8), // never 0, and <= MAX_HEADER_DEPTH
    ListItem,
    Code,
}

// hem. PR welcome
static SPACES: &'static str = "                                                                ";

/// a composite is a monoline sequence of compounds.
/// - the global style of the composite, if any
/// - a vector of styled parts
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Composite<'a> {
    pub style: CompositeStyle,
    pub compounds: Vec<Compound<'a>>,
}

impl<'a> From<Vec<Compound<'a>>> for Composite<'a> {
    fn from(compounds: Vec<Compound<'a>>) -> Composite<'a> {
        Composite {
            style: CompositeStyle::Paragraph,
            compounds,
        }
    }
}

impl<'a> Composite<'a> {
    pub fn new() -> Composite<'a> {
        Composite {
            style: CompositeStyle::Paragraph,
            compounds: Vec::new(),
        }
    }
    /// parse a monoline markdown snippet which isn't from a text.
    pub fn from_inline(md: &'a str) -> Composite<'a> {
        LineParser::from(md).inline()
    }
    #[inline(always)]
    pub fn is_code(&self) -> bool {
        match self.style {
            CompositeStyle::Code { .. } => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub fn is_list_item(&self) -> bool {
        match self.style {
            CompositeStyle::ListItem { .. } => true,
            _ => false,
        }
    }
    // return the total number of characters in the composite
    //
    // Example
    // ```rust
    // assert_eq!(Line::from("τ:`2π`").char_length(), 4);
    // ```
    //
    // This may not be the visible width: a renderer can
    //  add some things (maybe some caracters) to wrap inline code,
    //  or a bullet in front of a list item
    #[inline(always)]
    pub fn char_length(&self) -> usize {
        self.compounds
            .iter()
            .fold(0, |sum, compound| sum + compound.as_str().chars().count())
    }
    /// remove all white spaces at left, unless in inline code
    /// Empty compounds are cleaned out
    pub fn trim_left_spaces(&mut self) {
        loop {
            if self.compounds.len() == 0 {
                break;
            }
            if self.compounds[0].code {
                break;
            }
            self.compounds[0].trim_left_spaces();
            if self.compounds[0].is_empty() {
                self.compounds.remove(0);
            } else {
                break;
            }
        }
    }
    /// remove all white spaces at right, unless in inline code
    /// Empty compounds are cleaned out
    pub fn trim_right_spaces(&mut self) {
        loop {
            if self.compounds.len() == 0 {
                break;
            }
            let last = self.compounds.len() - 1;
            if self.compounds[last].code {
                break;
            }
            self.compounds[last].trim_right_spaces();
            if self.compounds[last].is_empty() {
                self.compounds.remove(last);
            } else {
                break;
            }
        }
    }
    pub fn trim_spaces(&mut self) {
        self.trim_left_spaces();
        self.trim_right_spaces();
    }
    pub fn is_empty(&self) -> bool {
        self.compounds.len() == 0
    }
    pub fn pad_left(&mut self, nb_added_spaces: usize) {
        if nb_added_spaces > 0 {
            let nb_added_spaces = nb_added_spaces.min(SPACES.len());
            self.compounds.insert(0, Compound::raw_part(&SPACES, 0, nb_added_spaces));
        }
    }
    pub fn pad_right(&mut self, nb_added_spaces: usize) {
        if nb_added_spaces > 0 {
            let nb_added_spaces = nb_added_spaces.min(SPACES.len());
            self.compounds.push(Compound::raw_part(&SPACES, 0, nb_added_spaces));
        }
    }
}
// Tests trimming composite
#[cfg(test)]
mod tests {
    use crate::composite::*;
    use crate::compound::*;

    #[test]
    fn composite_trim() {
        let mut left = Composite::from_inline(" *some* text  ");
        left.trim_spaces();
        assert_eq!(
            left,
            Composite {
                style: CompositeStyle::Paragraph,
                compounds: vec![
                    Compound::raw_str("some").italic(),
                    Compound::raw_str(" text"),
                ]
            }
        );
    }

    #[test]
    fn composite_trim_keep_code() {
        let mut left = Composite::from_inline(" ` `  ");
        left.trim_spaces();
        assert_eq!(
            left,
            Composite {
                style: CompositeStyle::Paragraph,
                compounds: vec![
                    Compound::raw_str(" ").code(),
                ]
            }
        );
    }

    #[test]
    fn empty_composite_trim() {
        let mut left = Composite::from_inline(" * * ** `` **  ");
        left.trim_left_spaces();
        assert_eq!(left.compounds.len(), 0);
    }
}
