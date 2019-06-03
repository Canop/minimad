use std::fmt::{self, Write};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Alignment {
    Unspecified,
    Left,
    Center,
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Unspecified
    }
}

/// a Compound is a part of a line with a consistent styling.
/// It can be part of word, several words, some inline code, or even the whole line.
#[derive(Clone)]
pub struct Compound<'s> {
    src: &'s str, // the source string from which the compound is a part
    start: usize, // start index in bytes, included
    end: usize,   // end index in bytes, excluded
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
}

impl<'s> Compound<'s> {
    // make a raw unstyled compound
    // Involves no parsing
    #[inline(always)]
    pub fn raw_str(src: &'s str) -> Compound<'s> {
        Compound {
            src,
            start: 0,
            end: src.len(),
            bold: false,
            italic: false,
            code: false,
        }
    }
    /// return a sub part of the compound, with the same syling
    /// r_start is relative, that is 0 is the index of the first
    /// char of this compound.
    #[inline(always)]
    pub fn sub(&self, r_start: usize, r_end: usize) -> Compound<'s> {
        Compound {
            src: self.src,
            start: self.start + r_start,
            end: self.start + r_end,
            bold: self.bold,
            italic: self.italic,
            code: self.code,
        }
    }
    /// return a sub part at end of the compound, with the same syling
    /// r_start is relative, that is if you give 0 you get a clone of
    /// this compound
    #[inline(always)]
    pub fn tail(&self, r_start: usize) -> Compound<'s> {
        Compound {
            src: self.src,
            start: self.start + r_start,
            end: self.end,
            bold: self.bold,
            italic: self.italic,
            code: self.code,
        }
    }
    // make a raw unstyled compound from part of a string
    // Involves no parsing
    #[inline(always)]
    pub fn raw_part(src: &'s str, start: usize, end: usize) -> Compound<'s> {
        Compound {
            src,
            start,
            end,
            bold: false,
            italic: false,
            code: false,
        }
    }
    #[inline(always)]
    pub fn new(
        src: &'s str, // the source string from which the compound is a part
        start: usize, // start index in bytes
        end: usize,
        bold: bool,
        italic: bool,
        code: bool,
    ) -> Compound<'s> {
        Compound {
            src,
            start,
            end,
            italic,
            bold,
            code,
        }
    }
    #[inline(always)]
    pub fn bold(mut self) -> Compound<'s> {
        self.bold = true;
        self
    }
    #[inline(always)]
    pub fn italic(mut self) -> Compound<'s> {
        self.italic = true;
        self
    }
    #[inline(always)]
    pub fn code(mut self) -> Compound<'s> {
        self.code = true;
        self
    }
    #[inline(always)]
    pub fn set_bold(&mut self, bold: bool) {
        self.bold = bold;
    }
    #[inline(always)]
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
    }
    #[inline(always)]
    pub fn set_code(&mut self, code: bool) {
        self.code = code;
    }
    #[inline(always)]
    pub fn as_str(&self) -> &'s str {
        &self.src[self.start..self.end]
    }
    #[inline(always)]
    pub fn char_length(&self) -> usize {
        self.as_str().chars().count()
    }
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
    pub fn trim_left_spaces(&mut self) {
        let mut chars = self.as_str().char_indices();
        let mut didx: usize;
        loop {
            if let Some((idx, char)) = chars.next() {
                didx = idx;
                if !char.is_whitespace() {
                    break;
                }
            } else {
                // the whole compound is made of white spaces
                self.start = self.end;
                return;
            }
        }
        self.start += didx;
    }
    pub fn trim_right_spaces(&mut self) {
        let mut chars = self.as_str().char_indices().rev();
        let mut didx = 0;
        loop {
            if let Some((idx, char)) = chars.next() {
                if !char.is_whitespace() {
                    break;
                }
                didx = idx;
            } else {
                // the whole compound is made of white spaces
                self.start = self.end;
                return;
            }
        }
        if didx > 0 {
            self.end = self.start + didx;
        }
    }
    pub fn trim_spaces(&mut self) {
        self.trim_left_spaces();
        self.trim_right_spaces();
    }
}

impl fmt::Display for Compound<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())?;
        Ok(())
    }
}

impl fmt::Debug for Compound<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.bold {
            f.write_char('B')?;
        }
        if self.italic {
            f.write_char('I')?;
        }
        if self.code {
            f.write_char('C')?;
        }
        f.write_char('"')?;
        f.write_str(self.as_str())?;
        f.write_char('"')?;
        Ok(())
    }
}

impl PartialEq for Compound<'_> {
    fn eq(&self, other: &Compound<'_>) -> bool {
        self.as_str() == other.as_str()
            && self.bold == other.bold
            && self.italic == other.italic
            && self.code == other.code
    }
}
impl Eq for Compound<'_> {}

#[test]
fn test_trim_left() {
    let mut left = Compound::raw_str(" ");
    left.trim_left_spaces();
    assert!(left.is_empty());

    let mut left = Compound::raw_str("  text");
    left.trim_left_spaces();
    assert_eq!(left, Compound::raw_str("text"), "trim 2 spaces");

    let mut left = Compound::raw_str("text");
    left.trim_left_spaces();
    assert_eq!(left, Compound::raw_str("text"), "not trimming when no space");
}

#[test]
fn test_trim_right() {
    let mut left = Compound::raw_str(" ");
    left.trim_right_spaces();
    assert!(left.is_empty());

    let mut left = Compound::raw_str("  text   ");
    left.trim_right_spaces();
    assert_eq!(left, Compound::raw_str("  text"));

    let mut left = Compound::raw_str("text");
    left.trim_right_spaces();
    assert_eq!(left, Compound::raw_str("text"), "not trimming when no space");
}
