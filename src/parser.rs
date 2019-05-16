
use crate::compound::{Compound};
use crate::line::*;

pub struct LineParser<'s> {
    src: &'s str,
    idx: usize,
    bold: bool,
    italic: bool,
    code: bool,
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
    fn close_compound(
        &mut self,
        end: usize,
        tag_length: usize,
        compounds: &mut Vec<Compound<'s>>,
    ) {
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
    pub fn line(&mut self) -> Line<'s> {
        assert_eq!(self.idx, 0, "LineParser.line() called more than once");
        let mut compounds = Vec::new();
        let mut after_first_star = false;
        for (idx, char) in self.src.char_indices() {
            if self.code {
                // only one thing matters: whether we're closing the inline code
                if char == '`' {
                    self.close_compound(idx, 1, &mut compounds);
                    self.code = false;
                }
            } else if after_first_star {
                match char {
                    '*' => { // this is the second star
                        self.close_compound(idx-1, 2, &mut compounds);
                        self.bold ^= true;
                    }
                    _ => { // there was only one star
                        // Note that we don't handle a tag just after a star (execpt in code)
                        self.close_compound(idx-1, 1, &mut compounds);
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
                        _ => {
                        }
                    }
            }
        }
        let mut idx = self.src.len();
        if after_first_star {
            idx -= 1;
        }
        self.close_compound(idx, 0, &mut compounds);
        Line {
            compounds,
        }
    }
}

// Tests of line parsing
#[cfg(test)]
mod tests {
    use crate::compound::*;
    use crate::line::*;

    #[test]
    fn simple_line_parsing() {
        assert_eq!(
            Line::from("Hello **World**. *Code*: `sqrt(π/2)`"),
            Line { compounds: vec![
                Compound::raw_str("Hello "),
                Compound::raw_str("World").bold(),
                Compound::raw_str(". "),
                Compound::raw_str("Code").italic(),
                Compound::raw_str(": "),
                Compound::raw_str("sqrt(π/2)").code(),
            ]}
        );
    }

    #[test]
    fn nested_styles_parsing() {
        assert_eq!(
            Line::from("*Italic then **bold and italic `and some *code*`** and italic*"),
            Line { compounds: vec![
                Compound::raw_str("Italic then ").italic(),
                Compound::raw_str("bold and italic ").bold().italic(),
                Compound::raw_str("and some *code*").bold().italic().code(),
                Compound::raw_str(" and italic").italic(),
            ]}
        );
    }
}
