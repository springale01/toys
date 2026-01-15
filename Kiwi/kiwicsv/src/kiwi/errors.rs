use std::borrow::Cow;

use thiserror::Error;

use crate::kiwi::traits::Deserialize;

#[derive(Debug, Error)]
pub enum KiwiError {
    // ----- Difference Errors -----
    #[error("Different Than Expected Type")]
    DifferentType {
        // Non Crucial
        given: String,
        inferred: String,
    },
    // ----- Token Errors -----
    #[error("Empty Content!")]
    NoContent,

    // ----- Runtime Errors -----
    #[error("Error From KiwiFloat!: {msg}")]
    KiwiFloatError { msg: String },
    #[error("Unknown!")]
    UnknownError,
    #[error("IncorrectFormat")]
    IncorrectFormat,
    #[error("Other")]
    OtherError { type_info: String, msg: String },
    #[error("Parsing Error")]
    ParseError,
}

pub type KiwiResult<T> = Result<T, KiwiError>;

impl KiwiError {
    /// Pretty print for display
    pub fn to_value<'a>(&'a self) -> Cow<'a, str> {
        match self {
            // ----- Display Errors -----
            Self::NoContent => Cow::Borrowed("[KiwiError::<NoContent>]"),
            Self::DifferentType { given, inferred } => Cow::Owned(format!(
                "KiwiError::<DifferentType>: | given {}, got {} |",
                given, inferred
            )),

            // ----- Runtime Errors -----
            Self::KiwiFloatError { msg } => {
                let message = format!("KiwiError::<FloatError>: | {} |", msg);
                Cow::Owned(message)
            }
            Self::UnknownError => format!("KiwiError::<UnkownError").into(),
            Self::IncorrectFormat => Cow::Borrowed("KiwiError::<IncorrectFormat>"),
            Self::OtherError { type_info, msg } => {
                Cow::Owned(format!("KiwiError::<{}>: | {} |", type_info, msg))
            }
            Self::ParseError => format!("KiwiError::<ParseError>").into(),
        }
    }
}

impl<'a> Deserialize<'a, KiwiError> for KiwiError {
    type Error = KiwiError;

    fn deserialize(content: &str) -> Result<KiwiError, KiwiError> {
        let content = content.trim();

        let body = content
            .strip_prefix("KiwiError::<")
            .ok_or(KiwiError::IncorrectFormat)?;

        let (kind, rest) = body.split_once('>').ok_or(KiwiError::IncorrectFormat)?;

        match kind.to_lowercase().as_str() {
            "parseerror" => Ok(KiwiError::ParseError),
            "unknownerror" => Ok(KiwiError::UnknownError),
            "incorrectformat" => Ok(KiwiError::IncorrectFormat),

            "floaterror" => {
                let msg = rest
                    .strip_prefix(": |")
                    .and_then(|s| s.strip_suffix('|'))
                    .ok_or(KiwiError::IncorrectFormat)?
                    .trim()
                    .to_string();

                Ok(KiwiError::KiwiFloatError { msg })
            }

            "other" => {
                let (type_info, msg) = rest
                    .strip_prefix(": |")
                    .and_then(|s| s.strip_suffix('|'))
                    .and_then(|s| s.split_once(':'))
                    .ok_or(KiwiError::IncorrectFormat)?;

                Ok(KiwiError::OtherError {
                    type_info: type_info.trim().to_string(),
                    msg: msg.trim().to_string(),
                })
            }

            _ => Err(KiwiError::IncorrectFormat),
        }
    }
}
