use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

mod convert;

pub struct NpxError {
    kind: Box<NpxErrorKind>,
}

#[derive(Debug)]
pub enum NpxErrorKind {
    Custom { message: String },
}

impl Error for NpxError {}

impl Debug for NpxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}
impl Display for NpxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}
impl Display for NpxErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NpxErrorKind::Custom { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl AsRef<NpxErrorKind> for NpxError {
    fn as_ref(&self) -> &NpxErrorKind {
        &self.kind
    }
}

impl AsMut<NpxErrorKind> for NpxError {
    fn as_mut(&mut self) -> &mut NpxErrorKind {
        &mut self.kind
    }
}
