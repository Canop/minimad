use std::cmp;
use crate::compound::Compound;
use crate::line::*;

/// The structure parsing a line.
/// Normally not used directly but though `line::from(str)`
pub struct LineParser<'s> {
    src: &'s str,
    idx: usize,
    bold: bool,
    italic: bool,
    code: bool,
}

/// count the number of '#' at start. Return 0 if they're
/// not followed by a ' ' or if they're too many
fn header_level(src: &str) -> usize {
    let src = src.as_bytes();
    let mut l: usize = src.len();
    if l > 2 {
        l = cmp::min(src.len()-1, 9);
        for i in 0..l {
            match src[i] {
                b'#' => {}
                b' ' => { return i; }
                _ => { return 0; }
            }
        }
    }
    0
}

#[test]
fn header_level_count() {
    assert_eq!(header_level(""), 0);
    assert_eq!(header_level("#"), 0);
    assert_eq!(header_level("# "), 0); // we don't allow empty headers
    assert_eq!(header_level("# A"), 1);
    assert_eq!(header_level(" "), 0);
    assert_eq!(header_level("test"), 0);
    assert_eq!(header_level("###b"), 0);
    assert_eq!(header_level("###"), 0);
    assert_eq!(header_level("### b"), 3);
    assert_eq!(header_level(" a b"), 0);
    assert_eq!(header_level("# titre"), 1);
    assert_eq!(header_level("#### *titre*"), 4);
    assert_eq!(header_level("######## a b"), 8);
    assert_eq!(header_level("######### a b"), 0); // too deep
}

impl<'s> LineParser<'s> {
    pub fn from(src: &'s str) -> LineParser {
        LineParser {
            src,
            idx: 0,
            bold: false,
            italic: false,
            code: false,
        }
    }
    fn close_compound(&mut self, end: usize, tag_length: usize, compounds: &mut Vec<Compound<'s>>) {
        if end > self.idx {
            compounds.push(Compound::new(
                &self.src,
                self.idx,
                end,
                self.bold,
                self.italic,
                self.code,
            ));
        }
        self.idx = end + tag_length;
    }
    fn code_compound_from_idx(&self, idx: usize) -> Vec<Compound<'s>> {
        vec![ Compound::new(
            &self.src,
            idx,
            self.src.len(),
            false,
            false,
            true,
        )]
    }
    fn parse_compounds(&mut self) -> Vec<Compound<'s>> {
        let mut compounds = Vec::new();
        let mut after_first_star = false;
        for (idx, char) in self.src.char_indices().skip(self.idx) {
            if self.code {
                // only one thing matters: whether we're closing the inline code
                if char == '`' {
                    self.close_compound(idx, 1, &mut compounds);
                    self.code = false;
                }
            } else if after_first_star {
                match char {
                    '*' => {
                        // this is the second star
                        self.close_compound(idx - 1, 2, &mut compounds);
                        self.bold ^= true;
                    }
                    _ => {
                        // there was only one star
                        // Note that we don't handle a tag just after a star (execpt in code)
                        self.close_compound(idx - 1, 1, &mut compounds);
                        self.italic ^= true;
                    }
                }
                after_first_star = false;
            } else {
                match char {
                    '*' => {
                        after_first_star = true;
                        // we don't know yet if it's one or two stars
                    }
                    '`' => {
                        self.close_compound(idx, 1, &mut compounds);
                        self.code = true;
                    }
                    _ => {}
                }
            }
        }
        let mut idx = self.src.len();
        if after_first_star {
            idx -= 1;
        }
        self.close_compound(idx, 0, &mut compounds);
        compounds
    }
    pub fn line(&mut self) -> Line<'s> {
        assert_eq!(self.idx, 0, "LineParser.line() called more than once");
        if self.src.starts_with("    ") {
            return Line {
                style: LineStyle::Code,
                compounds: self.code_compound_from_idx(4)
            }
        }
        if self.src.starts_with("\t") {
            return Line {
                style: LineStyle::Code,
                compounds: self.code_compound_from_idx(1)
            }
        }
        if self.src.starts_with("* ") {
            self.idx = 2;
            return Line {
                style: LineStyle::ListItem,
                compounds: self.parse_compounds(),
            }
        }
        let header_level = header_level(self.src);
        if header_level > 0 {
            self.idx = header_level + 1;
            return Line {
                style: LineStyle::Header(header_level as u8),
                compounds: self.parse_compounds(),
            }
        }
        Line {
            style: LineStyle::Normal,
            compounds: self.parse_compounds(),
        }
    }
}

/// Tests of line parsing
#[cfg(test)]
mod tests {
    use crate::compound::*;
    use crate::line::*;

    #[test]
    fn simple_line_parsing() {
        assert_eq!(
            Line::from("Hello **World**. *Code*: `sqrt(π/2)`"),
            Line {
                style: LineStyle::Normal,
                compounds: vec![
                    Compound::raw_str("Hello "),
                    Compound::raw_str("World").bold(),
                    Compound::raw_str(". "),
                    Compound::raw_str("Code").italic(),
                    Compound::raw_str(": "),
                    Compound::raw_str("sqrt(π/2)").code(),
                ]
            }
        );
    }

    #[test]
    fn nested_styles_parsing() {
        assert_eq!(
            Line::from("*Italic then **bold and italic `and some *code*`** and italic*"),
            Line {
                style: LineStyle::Normal,
                compounds: vec![
                    Compound::raw_str("Italic then ").italic(),
                    Compound::raw_str("bold and italic ").bold().italic(),
                    Compound::raw_str("and some *code*").bold().italic().code(),
                    Compound::raw_str(" and italic").italic(),
                ]
            }
        );
    }

    #[test]
    fn line_of_code() {
        assert_eq!(
            Line::from("    let r = Math.sin(π/2) * 7"),
            Line {
                style: LineStyle::Code,
                compounds: vec![
                    Compound::raw_str("let r = Math.sin(π/2) * 7").code(),
                ]
            }
        );
    }

    #[test]
    fn standard_header() {
        assert_eq!(
            Line::from("### just a title"),
            Line {
                style: LineStyle::Header(3),
                compounds: vec![
                    Compound::raw_str("just a title"),
                ]
            }
        );
    }

    #[test]
    fn list_item() {
        assert_eq!(
            Line::from("* *list* item"),
            Line {
                style: LineStyle::ListItem,
                compounds: vec![
                    Compound::raw_str("list").italic(),
                    Compound::raw_str(" item"),
                ]
            }
        );
    }

    #[test]
    fn styled_header() {
        assert_eq!(
            Line::from("## a header with some **bold**!"),
            Line {
                style: LineStyle::Header(2),
                compounds: vec![
                    Compound::raw_str("a header with some "),
                    Compound::raw_str("bold").bold(),
                    Compound::raw_str("!"),
                ]
            }
        );
    }
}
