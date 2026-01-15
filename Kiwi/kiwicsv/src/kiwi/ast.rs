use kiwi_utils::kiwi_float::KiwiFloat;

use crate::kiwi::errors::KiwiError;

#[derive(Debug)]
pub struct KiwiCSV {
    /// Names of each column
    pub header: Vec<String>,
    /// Each Cell is a item
    pub content: Vec<Vec<KiwiFruit>>,
    /// Majority of the type in the column
    pub footer: Vec<KiwiType>,
}

#[derive(Debug)]
pub enum KiwiType {
    Int(isize),
    Float(KiwiFloat),
    String(String),
    NaN,
    Error(KiwiError),
    Other { type_info: String, msg: String },
    Unknown,
}

#[derive(Debug)]
/// It combines Kiwitype and KiwiError
pub enum KiwiFruit {
    Type(KiwiType),
    Error(KiwiError),
}
