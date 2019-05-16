use std::fmt::{self, Write};

/// a Compound is a part of a line with a consistent styling.
/// It can be part of word, several words, some inline code, or even the whole line.

pub struct Compound<'s> {
    src: &'s str, // the source string from which the compound is a part
    start: usize, // start index in bytes
    end: usize,
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
}

impl<'s> Compound<'s> {
    // make a raw unstyled compound, involves no parsing
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
    // make a raw unstyled compound, involves no parsing
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
    pub fn new(
        src: &'s str, // the source string from which the compound is a part
        start: usize, // start index in bytes
        end: usize,
        bold: bool,
        italic: bool,
        code: bool,
    ) -> Compound<'s> {
        Compound {
            src, start, end, italic, bold, code,
        }
    }
    pub fn bold(mut self) -> Compound<'s> {
        self.bold = true;
        self
    }
    pub fn italic(mut self) -> Compound<'s> {
        self.italic = true;
        self
    }
    pub fn code(mut self) -> Compound<'s> {
        self.code = true;
        self
    }
    pub fn set_bold(&mut self, bold: bool) {
        self.bold = bold;
    }
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
    }
    pub fn set_code(&mut self, code: bool) {
        self.code = code;
    }
    pub fn as_str(&self) -> &'s str {
        &self.src[self.start..self.end]
    }
}

impl fmt::Display for Compound<'_> {
    fn fmt(&self, f: & mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())?;
        Ok(())
    }
}

impl fmt::Debug for Compound<'_> {
    fn fmt(&self, f: & mut fmt::Formatter) -> fmt::Result {
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
    fn eq(&self, other: &Compound) -> bool {
        self.as_str() == other.as_str()
        && self.bold == other.bold
        && self.italic == other.italic
        && self.code == other.code
    }
}
impl Eq for Compound<'_> {
}
