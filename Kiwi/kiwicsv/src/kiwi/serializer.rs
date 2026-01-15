use std::borrow::Cow;

use kiwi_utils::kiwi_float::KiwiFloat;

use crate::kiwi::{
    ast::{KiwiCSV, KiwiFruit, KiwiType},
    errors::{KiwiError, KiwiResult},
    inferencer::{PLACES_AFTERDECIMAL, TOTALDIGITS},
    parser::parse,
    tokenizer::KiwiTokenizer,
    traits::{Deserialize, Serialize},
};

// ----- KiwiCSV ------
impl KiwiCSV {
    /// Make sure stuff is Tokenked using KiwiTokenizer before
    pub fn new(content: Vec<Vec<Cow<str>>>, header: Option<Vec<Cow<str>>>) -> KiwiResult<Self> {
        if content.is_empty() {
            return Err(crate::kiwi::errors::KiwiError::NoContent);
        }

        let body = parse(content);

        let header = if let Some(header) = header {
            header
        } else {
            vec![Cow::Borrowed("EmptyHeader")]
        };

        todo!()
    }
}

impl std::fmt::Display for KiwiCSV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = self.header.join(", ");
        let body = self
            .content
            .iter()
            .map(|line| {
                line.iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        let footer = self
            .footer
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}\n{}\n{}", header, body, footer)
    }
}
// ----- KiwiType -----

impl std::fmt::Display for KiwiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let thing = match self {
            Self::String(x) => format!("[Kiwi::<String>: | {} |]", x),
            Self::Float(x) => format!("[Kiwi::<Float>: | {} |]", x.to_string()),
            Self::Error(x) => format!("{}", x.to_value()),
            Self::Int(x) => format!("[Kiwi::<Integer>: | {} |]", x),
            Self::NaN => format!("[Kiwi::<NaN>]"),
            Self::Unknown => format!("[Kiwi::<Unknown>]"),
            Self::Other { type_info, msg } => {
                format!("[Kiwi::<Other: {}>: | {} |", type_info, msg)
            }
        };

        write!(f, "{}", thing)
    }
}
impl<'a> Serialize<'a> for KiwiType {
    fn serialize(&self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }
}
impl<'a> Deserialize<'a, KiwiType> for KiwiType {
    type Error = KiwiError;
    fn deserialize(content: &str) -> Result<KiwiType, KiwiError> {
        let content = content.trim();

        let body = content
            .strip_prefix("[Kiwi::<")
            .ok_or(KiwiError::IncorrectFormat)?;

        let (mut type_part, rest) = body
            .split_once(">: |")
            .map(|(x, y)| (x.trim(), y))
            .ok_or(KiwiError::IncorrectFormat)?;

        let binding = type_part.to_lowercase();
        type_part = &binding;
        let (value_part, _) = rest
            .split_once("|]")
            .map(|(x, thing)| (x.to_lowercase().trim().to_string(), thing))
            .ok_or(KiwiError::IncorrectFormat)?;

        // check if Other is present
        if type_part.starts_with("other: ") {
            let type_info = type_part
                .strip_prefix("other: ")
                .ok_or(KiwiError::IncorrectFormat)?;
            let msg = value_part.trim();
            return Ok(KiwiType::Other {
                type_info: type_info.into(),
                msg: msg.into(),
            });
        }

        match type_part {
            "string" => {
                return Ok(KiwiType::String(value_part));
            }
            "float" => {
                return Ok(KiwiType::Float({
                    KiwiFloat::new(
                        value_part
                            .parse::<f64>()
                            .map_err(|e| KiwiError::KiwiFloatError { msg: e.to_string() })?,
                        TOTALDIGITS,
                        PLACES_AFTERDECIMAL,
                    )
                    .map_err(|e| KiwiError::KiwiFloatError { msg: e.to_value() })?
                }));
            }
            "error" => {
                let err = KiwiError::deserialize(&value_part)?;
                return Ok(KiwiType::Error(err));
            }
            "integer" => {
                return Ok(KiwiType::Int(
                    value_part
                        .parse::<isize>()
                        .map_err(|_| KiwiError::ParseError)?,
                ));
            }
            "nan" => {
                return Ok(KiwiType::NaN);
            }
            "unknown" => {
                return Ok(KiwiType::Unknown);
            }
            _ => {
                return Ok(KiwiType::Other {
                    type_info: type_part.into(),
                    msg: value_part,
                });
            }
        }
    }
}

// ----- KiwiFruit -----
impl std::fmt::Display for KiwiFruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringed = match self {
            Self::Error(e) => e.to_value().to_owned(),
            Self::Type(thing) => thing.to_string().into(),
        };

        write!(f, "{}", stringed)
    }
}

#[cfg(test)]
mod tests {
    use kiwi_utils::kiwi_float::KiwiFloat;

    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            KiwiType::String("joe".to_string()).to_string(),
            "joe".to_string()
        );

        println!(
            "{}",
            KiwiType::Float(KiwiFloat::new(68.684, 100, 30).unwrap())
        )
    }
    #[test]
    fn test_deserializatoin() {
        let test_thing = "[Kiwi::<Integer>: | 74 |]";

        let out = KiwiType::deserialize(test_thing).unwrap_or(KiwiType::NaN);

        println!("{}", out)
    }
}
