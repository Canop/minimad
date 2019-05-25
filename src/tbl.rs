use crate::composite::Composite;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TableRow<'a> {
    pub cells: Vec<Composite<'a>>,
}

impl<'a> TableRow<'a> {}
