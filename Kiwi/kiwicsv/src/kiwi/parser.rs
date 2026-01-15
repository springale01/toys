use std::borrow::Cow;

use crate::kiwi::{
    ast::KiwiFruit,
    errors::{KiwiError, KiwiResult},
    inferencer::{self, infer_type},
};
/// Parses a already tokened 2d array of words into a 2d array of inferred KiwiFruits
pub fn parse<'a>(array: Vec<Vec<Cow<'a, str>>>) -> Vec<Vec<KiwiFruit>> {
    array
        .iter()
        .map(|line| {
            line.iter()
                .map(|item| match infer_type(item.as_ref()) {
                    Ok(kiwitype) => KiwiFruit::Type(kiwitype),
                    Err(e) => KiwiFruit::Error(e),
                })
                .collect::<Vec<KiwiFruit>>()
        })
        .collect::<Vec<Vec<KiwiFruit>>>()
}
