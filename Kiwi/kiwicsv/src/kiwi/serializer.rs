use std::borrow::Cow;

use kiwi_utils::kiwi_float::KiwiFloat;

use crate::kiwi::{
    ast::{KiwiCSV, KiwiFruit, KiwiType},
    errors::{KiwiError, KiwiResult},
    inferencer::{PLACES_AFTERDECIMAL, TOTALDIGITS},
    parser::parse,
    tokenizer::{self, KiwiTokenizer},
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

    pub fn pretty_print(&self, tokenizer: KiwiTokenizer) -> String {
        let header = self.header.join(", ");
        let body = self
            .content
            .iter()
            .map(|line| {
                line.iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join(tokenizer.delimiter)
            })
            .collect::<Vec<String>>()
            .join("\n");

        let footer = self.footer.as_ref().map(|footer| {
            footer
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(tokenizer.delimiter)
        });

        match footer {
            Some(footer) if !footer.is_empty() => {
                format!("{}\n{}\n{}", header, body, footer)
            }
            _ => {
                format!("{}\n{}", header, body)
            }
        }
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

        let footer = self.footer.as_ref().map(|footer| {
            footer
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        });

        match footer {
            Some(footer) if !footer.is_empty() => {
                write!(f, "{}\n{}\n{}", header, body, footer)
            }
            _ => {
                write!(f, "{}\n{}", header, body)
            }
        }
    }
}

// ----- KiwiType -----

impl std::fmt::Display for KiwiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let thing = match self {
            Self::String(x) => format!("[Kiwi::<String>: | {} |]", x),
            Self::Float(x) => format!("[Kiwi::<Float>: | {} |]", x.to_string()),
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
    fn serialize(&self) -> Cow<'_, str> {
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
                let first_upper = uppercase_first_letter(&value_part);
                return Ok(KiwiType::String(first_upper));
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

fn uppercase_first_letter(string: &str) -> String {
    let mut charz = string.chars();
    match charz.next() {
        Some(c) => {
            format!("{}{}", c.to_uppercase(), charz.as_str())
        }
        None => String::new(),
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

impl<'a> Serialize<'a> for KiwiFruit {
    fn serialize(&'_ self) -> Cow<'_, str> {
        match self {
            Self::Type(kiwitype) => kiwitype.serialize(),
            Self::Error(error) => error.serialize(),
        }
    }
}

impl<'a> Deserialize<'a, KiwiFruit> for KiwiFruit {
    type Error = KiwiError;
    fn deserialize(content: &str) -> Result<KiwiFruit, KiwiError> {
        match KiwiType::deserialize(content) {
            Ok(v) => return Ok(KiwiFruit::Type(v)),
            Err(_) => {}
        }

        match KiwiError::deserialize(content) {
            Ok(e) => return Ok(KiwiFruit::Error(e)),
            Err(_) => {}
        }

        Err(KiwiError::UnknownError)
    }
}

// ----- tests -----
#[cfg(test)]
mod tests {
    use kiwi_utils::kiwi_float::KiwiFloat;

    use super::*;
    #[test]
    fn test_deserializatoin() {
        let test_thing = "[Kiwi::<Integer>: | 74 |]";
        let out = KiwiType::deserialize(test_thing).expect("failed to deserialize type!");

        assert_eq!(test_thing, KiwiType::Int(74).to_string());

        println!("{out}")
    }
}
