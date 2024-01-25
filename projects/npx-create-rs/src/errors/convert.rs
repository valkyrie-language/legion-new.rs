#[cfg(feature = "wasm-opt")]
use wasm_opt::OptimizationError;
use crate::{NpxError, errors::NpxErrorKind};

impl From<std::io::Error> for NpxError {
    fn from(error: std::io::Error) -> Self {
        let kind = NpxErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}

impl From<anyhow::Error> for NpxError {
    fn from(error: anyhow::Error) -> Self {
        let kind = NpxErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}


impl From<dialoguer::Error> for NpxError {
    fn from(error: dialoguer::Error) -> Self {
        let kind = NpxErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}



#[cfg(feature = "wasm-opt")]
impl From<OptimizationError> for NpxError {
    fn from(error: OptimizationError) -> Self {
        let kind = NpxErrorKind::Custom { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}
