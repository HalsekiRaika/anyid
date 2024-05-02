use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Downcast,
}

#[rustfmt::skip]
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Downcast => write!(f, "Downcast from Any failed. Wrong type may have been used."),
        }
    }
}

impl std::error::Error for Error {}
