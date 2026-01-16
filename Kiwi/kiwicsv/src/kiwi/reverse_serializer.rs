use std::str::FromStr;

use kiwi_utils::kiwi_float::KiwiFloat;

use crate::kiwi::{
    ast::{KiwiCSV, KiwiFruit, KiwiSettings, KiwiType},
    errors::{KiwiError, KiwiResult},
    inferencer::{PLACES_AFTERDECIMAL, TOTALDIGITS},
    parser::{parse, parse_formatted},
    tokenizer::KiwiTokenizer,
    traits::Deserialize,
};

impl KiwiCSV {
    fn from_csv(
        content: &str,
        tokenizer: KiwiTokenizer,
        settings: KiwiSettings,
    ) -> KiwiResult<Self> {
        let mut tokened = match tokenizer.tokenize(content) {
            Ok(token) => token,
            Err(e) => return Err(e),
        };
        // grabs the first line as header if header is present in settings
        let header: Vec<String> = if settings.header {
            let header = if let Some(row) = tokened.drain(0..1).next() {
                row.into_iter().map(|c| c.to_string()).collect()
            } else {
                vec![]
            };

            header
        } else {
            vec![]
        };
        // grabs the footer if the footer is present
        let footer = if settings.footer {
            if let Some(foot) = tokened.pop() {
                Some(
                    foot.into_iter()
                        .map(|cell| {
                            KiwiFruit::deserialize(&cell).unwrap_or(KiwiFruit::Error(
                                KiwiError::OtherError {
                                    type_info: "footer".into(),
                                    msg: "Failed to parse footer".into(),
                                },
                            ))
                        })
                        .collect::<Vec<KiwiFruit>>(),
                )
            } else {
                return Err(KiwiError::NoContent);
            }
        } else {
            None
        };
        //rest is body
        let body = parse(tokened);

        Ok(Self {
            header,
            content: body,
            footer,
        })
    }

    // ----- Serialization/Deserialization -----
    pub fn deseralize_already_serialized(
        content: &str,
        tokenizer: KiwiTokenizer,
        settings: KiwiSettings,
    ) -> KiwiResult<Self> {
        let mut tokened = match tokenizer.tokenize(content) {
            Ok(token) => token,
            Err(e) => return Err(e),
        };
        let header: Vec<String> = if settings.header {
            let header = if let Some(row) = tokened.drain(0..1).next() {
                row.into_iter().map(|c| c.to_string()).collect()
            } else {
                vec![]
            };

            header
        } else {
            vec![]
        };
        let footer = if settings.footer {
            if let Some(foot) = tokened.pop() {
                Some(
                    foot.into_iter()
                        .map(|cell| {
                            KiwiFruit::deserialize(&cell).unwrap_or(KiwiFruit::Error(
                                KiwiError::OtherError {
                                    type_info: "footer".into(),
                                    msg: "Failed to parse footer".into(),
                                },
                            ))
                        })
                        .collect::<Vec<KiwiFruit>>(),
                )
            } else {
                return Err(KiwiError::NoContent);
            }
        } else {
            None
        };

        let body = parse_formatted(tokened);

        Ok(Self {
            header,
            content: body,
            footer,
        })
    }
}

#[test]
fn test_from_csv() {
    let settings = KiwiSettings {
        header: true,
        footer: false,
    };

    let thing = "id,name,score
    1,Alice,98.5
    2,Bob,72
    3,Charlie,NaN
    ";

    let tokenizer1 = KiwiTokenizer::new()
        .with_delim(",")
        .with_fill(true)
        .with_trim(true);

    let out = KiwiCSV::from_csv(thing, tokenizer1, settings).expect("failed to parse");

    println!("{}", out.to_string())
}

#[test]
fn test_round_trip_csv() {
    let settings = KiwiSettings {
        header: true,
        footer: false,
    };

    let input = r#"id,name,score
1,Alice,98.5
2,Bob,72
3,Charlie,NaN"#;

    let tokenizer = KiwiTokenizer::new()
        .with_delim(",")
        .with_fill(true)
        .with_trim(true);

    // CSV → KiwiCSV
    let parsed = KiwiCSV::from_csv(input, tokenizer.clone(), settings.clone())
        .expect("failed to parse input csv");

    println!("{}", &parsed);

    // KiwiCSV → CSV
    let output = parsed.pretty_print(tokenizer.clone());

    println!("{}", &output);
    // CSV → KiwiCSV AGAIN
    let reparsed = KiwiCSV::deseralize_already_serialized(&output, tokenizer, settings.clone())
        .expect("failed to parse round-tripped csv");

    println!("{}", &reparsed);
    // Structural equality check
    assert_eq!(parsed.header, reparsed.header);
    assert_eq!(parsed.content, reparsed.content);
    assert_eq!(parsed.footer, reparsed.footer);
}
