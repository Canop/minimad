/// Markdown parsing options
#[derive(Debug, Clone, Copy)]
pub struct Options {
    /// Remove one or several superfluous levels of indentations
    ///
    /// This is useful when your text is too deeply intended, for
    /// example because it's defined in a raw literal:
    ///
    /// ```
    /// use minimad::*;
    /// let text = r#"
    ///     bla bla bla
    ///     * item 1
    ///     * item 2
    /// "#;
    /// assert_eq!(
    ///     parse_text(text, Options { clean_indentations: true, ..Default::default() }),
    ///     Text { lines: vec![
    ///         Line::from("bla bla bla"),
    ///         Line::from("* item 1"),
    ///         Line::from("* item 2"),
    ///     ]},
    /// );
    /// ```
    ///
    pub clean_indentations: bool,
    pub continue_inline_code: bool,
    pub continue_italic: bool,
    pub continue_bold: bool,
    pub continue_strikeout: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Options {
    fn default() -> Self {
        Self {
            clean_indentations: false,
            continue_inline_code: false,
            continue_italic: false,
            continue_bold: false,
            continue_strikeout: false,
        }
    }
}

impl Options {
    pub fn clean_indentations(
        mut self,
        value: bool,
    ) -> Self {
        self.clean_indentations = value;
        self
    }
    pub fn continue_inline_code(
        mut self,
        value: bool,
    ) -> Self {
        self.continue_inline_code = value;
        self
    }
    pub fn continue_italic(
        mut self,
        value: bool,
    ) -> Self {
        self.continue_italic = value;
        self
    }
    pub fn continue_bold(
        mut self,
        value: bool,
    ) -> Self {
        self.continue_bold = value;
        self
    }
    pub fn continue_strikeout(
        mut self,
        value: bool,
    ) -> Self {
        self.continue_strikeout = value;
        self
    }
    pub fn continue_spans(
        mut self,
        value: bool,
    ) -> Self {
        self.continue_inline_code = value;
        self.continue_italic = value;
        self.continue_bold = value;
        self.continue_strikeout = value;
        self
    }
}
