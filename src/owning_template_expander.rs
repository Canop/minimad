use super::*;

/// A template expander owning the value you set
/// so that you don't have to keep them around until
/// you produce the text to display.
/// Additionnaly, the same expander can be used for several
/// templates.
pub struct OwningTemplateExpander<'s> {
    ops: Vec<FillingOperation<'s>>,
}
pub struct OwningSubTemplateExpander<'s> {
    ops: Vec<SubFillingOperation<'s>>,
}

enum FillingOperation<'s> {
    Set {
        name: &'s str,
        value: String,
    },
    SetMD {
        name: &'s str,
        value: String,
    },
    SetLines {
        name: &'s str,
        value: String,
    },
    SetLinesMD {
        name: &'s str,
        value: String,
    },
    Sub {
        name: &'s str,
        sub_expander: OwningSubTemplateExpander<'s>,
    },
}
enum SubFillingOperation<'s> {
    Set { name: &'s str, value: String },
    SetMD { name: &'s str, value: String },
}

impl<'s> OwningTemplateExpander<'s> {
    pub fn new() -> Self {
        let ops = Vec::new();
        Self { ops }
    }

    /// replace placeholders with name `name` with the given value, non interpreted
    /// (i.e. stars, backquotes, etc. don't mess the styling defined by the template)
    pub fn set<S: Into<String>>(&mut self, name: &'s str, value: S) -> &mut Self {
        self.ops.push(FillingOperation::Set {
            name,
            value: value.into(),
        });
        self
    }

    /// replace placeholders with name `name` with the given value, interpreted as markdown
    pub fn set_md<S: Into<String>>(&mut self, name: &'s str, value: S) -> &mut Self {
        self.ops.push(FillingOperation::SetMD {
            name,
            value: value.into(),
        });
        self
    }

    /// return a sub template expander. You can do set and set_md
    /// on the returned sub to fill an instance of the repeation section.
    pub fn sub(&mut self, name: &'s str) -> &mut OwningSubTemplateExpander<'s> {
        let idx = self.ops.len();
        self.ops.push(FillingOperation::Sub {
            name,
            sub_expander: OwningSubTemplateExpander::new(),
        });
        match &mut self.ops[idx] {
            FillingOperation::Sub {
                name: _,
                sub_expander,
            } => sub_expander,
            _ => unreachable!(),
        }
    }

    /// replace a placeholder with several lines.
    /// This is mostly useful when the placeholder is a repeatable line (code, list item)
    pub fn set_lines<S: Into<String>>(&mut self, name: &'s str, raw_lines: S) -> &mut Self {
        self.ops.push(FillingOperation::SetLines {
            name,
            value: raw_lines.into(),
        });
        self
    }

    /// replace a placeholder with several lines interpreted as markdown
    pub fn set_lines_md<S: Into<String>>(&mut self, name: &'s str, md: S) -> &mut Self {
        self.ops.push(FillingOperation::SetLinesMD {
            name,
            value: md.into(),
        });
        self
    }

    /// build a text by applying the replacements to the initial template
    pub fn expand<'t>(&'s self, template: &'t TextTemplate<'s>) -> Text<'s> {
        let mut expander = template.expander();
        for op in &self.ops {
            match op {
                FillingOperation::Set { name, value } => {
                    expander.set(name, &value);
                }
                FillingOperation::SetMD { name, value } => {
                    expander.set_md(name, &value);
                }
                FillingOperation::SetLines { name, value } => {
                    expander.set_lines(name, &value);
                }
                FillingOperation::SetLinesMD { name, value } => {
                    expander.set_lines_md(name, &value);
                }
                FillingOperation::Sub { name, sub_expander } => {
                    let sub = expander.sub(name);
                    for op in &sub_expander.ops {
                        match op {
                            SubFillingOperation::Set { name, value } => {
                                sub.set(name, &value);
                            }
                            SubFillingOperation::SetMD { name, value } => {
                                sub.set_md(name, &value);
                            }
                        }
                    }
                }
            }
        }
        expander.expand()
    }
}

impl<'s> OwningSubTemplateExpander<'s> {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }
    /// replace placeholders with name `name` with the given value, non interpreted
    /// (i.e. stars, backquotes, etc. don't mess the styling defined by the template)
    pub fn set<S: Into<String>>(&mut self, name: &'s str, value: S) -> &mut Self {
        self.ops.push(SubFillingOperation::Set {
            name,
            value: value.into(),
        });
        self
    }

    /// replace placeholders with name `name` with the given value, interpreted as markdown
    pub fn set_md<S: Into<String>>(&mut self, name: &'s str, value: S) -> &mut Self {
        self.ops.push(SubFillingOperation::SetMD {
            name,
            value: value.into(),
        });
        self
    }
}
