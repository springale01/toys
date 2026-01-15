use kiwi_utils::kiwi_float::KiwiFloat;

use crate::kiwi::{
    ast::KiwiType,
    errors::{KiwiError, KiwiResult},
};
// Edit this later for easier access
pub const TOTALDIGITS: usize = 30;
pub const PLACES_AFTERDECIMAL: usize = 12;

/// Note that the content passed onto this has to be a single thing
/// * if you pass not a single thing it might get inferred wrong
pub fn infer_type(content: &str) -> KiwiResult<KiwiType> {
    if content.is_empty() {
        return Ok(KiwiType::NaN);
    }

    if let Some(int) = content.parse::<isize>().ok() {
        return Ok(KiwiType::Int(int));
    }

    if let Some(float) = content.parse::<f64>().ok() {
        let kiwifloat = KiwiFloat::new(float, TOTALDIGITS, PLACES_AFTERDECIMAL)
            .map_err(|e| KiwiError::KiwiFloatError { msg: e.to_value() })?;
        return Ok(KiwiType::Float(kiwifloat));
    }

    Ok(KiwiType::String(content.to_string()))
}
