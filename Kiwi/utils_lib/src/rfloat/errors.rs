use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RFloatErrors {
    #[error("The value of the float is infinity!")]
    ValueIsInfinity,
    #[error("The value of the float is NAN!")]
    ValueIsNan,
    #[error("Something happened and we don't know what")]
    UnknownError,
    #[error("Failed to Parse this {thing} to {target}!")]
    ParseError { thing: String, target: String },
}

pub type KiwiFloatResult<T> = Result<T, RFloatErrors>;
