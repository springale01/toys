use std::borrow::Cow;

use thiserror::Error;
#[derive(Debug, Error)]
pub enum KiwiFloatError {
    #[error("The value of the float is infinity!")]
    ValueIsInfinity,
    #[error("The value of the float is NAN!")]
    ValueIsNan,
    #[error("Something happened and we don't know what")]
    UnknownError,
    #[error("Failed to Parse this {thing} to {target}!")]
    ParseError { thing: String, target: String },
}

pub type KiwiFloatResult<T> = Result<T, KiwiFloatError>;

impl KiwiFloatError {
    pub fn to_value(&self) -> String {
        match self {
            Self::ParseError { thing, target } => {
                format!("ParseError({},{})", thing, target)
            }
            Self::UnknownError => "UnknownError".to_string(),
            Self::ValueIsInfinity => "ValueIsInfinity".to_string(),
            Self::ValueIsNan => "NaN".to_string(),
        }
    }
}
