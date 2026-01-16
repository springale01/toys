use kiwi_utils::kiwi_float::KiwiFloat;

use crate::kiwi::errors::KiwiError;

#[derive(Debug)]
pub struct KiwiCSV {
    /// Names of each column
    pub header: Vec<String>,
    /// Each Cell is a item
    pub content: Vec<Vec<KiwiFruit>>,
    /// Majority of the type in the column
    pub footer: Option<Vec<KiwiFruit>>,
}

#[derive(Debug, PartialEq)]
pub enum KiwiType {
    Int(isize),
    Float(KiwiFloat),
    String(String),
    NaN,
    Other { type_info: String, msg: String },
    Unknown,
}

#[derive(Debug, PartialEq)]
/// It combines Kiwitype and KiwiError
pub enum KiwiFruit {
    Type(KiwiType),
    Error(KiwiError),
}

#[derive(Debug, Clone)]
pub struct KiwiSettings {
    pub header: bool,
    pub footer: bool,
}
