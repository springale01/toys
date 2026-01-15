use std::str::FromStr;

use kiwi_utils::kiwi_float::KiwiFloat;

use crate::kiwi::{
    ast::{KiwiFruit, KiwiType},
    errors::KiwiError,
    inferencer::{PLACES_AFTERDECIMAL, TOTALDIGITS},
};
