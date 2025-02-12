use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

use pyo3::{
    exceptions::{PyException, PyValueError},
    prelude::*,
};

#[derive(Debug)]
pub enum StrompyError {
    Json(&'static str),
    Struson(struson::reader::ReaderError),
    ParseFloat(ParseFloatError),
    ParseInt(std::num::ParseIntError),
}

impl Display for StrompyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrompyError::Json(e) => write!(f, r#"Unexpected key encountered, expected "{e}""#),
            StrompyError::Struson(e) => write!(f, "Struson error: {e}"),
            StrompyError::ParseFloat(e) => write!(f, "ParseFloat error: {e}"),
            StrompyError::ParseInt(e) => write!(f, "ParseInt error: {e}"),
        }
    }
}

impl IntoPy<Py<PyAny>> for StrompyError {
    fn into_py(self, py: pyo3::prelude::Python<'_>) -> Py<PyAny> {
        self.to_string().into_py(py)
    }
}

impl From<StrompyError> for pyo3::PyErr {
    fn from(e: StrompyError) -> Self {
        match e {
            StrompyError::ParseFloat(e) => PyValueError::new_err(e),
            StrompyError::ParseInt(e) => PyValueError::new_err(e),
            e => PyException::new_err(e),
        }
    }
}

impl From<struson::reader::ReaderError> for StrompyError {
    fn from(e: struson::reader::ReaderError) -> Self {
        Self::Struson(e)
    }
}

impl From<ParseFloatError> for StrompyError {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloat(e)
    }
}

impl From<ParseIntError> for StrompyError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}
