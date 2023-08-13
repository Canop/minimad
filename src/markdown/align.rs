/// Left, Center, Right or Unspecified
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Alignment {
    #[default]
    Unspecified,
    Left,
    Center,
    Right,
}

impl Alignment {
    pub fn col_spec(self) -> &'static str {
        match self {
            Self::Left => "|:-",
            Self::Right => "|-:",
            Self::Center => "|:-:",
            Self::Unspecified => "|-",
        }
    }
}
