use std::hash::Hash;

use crate::{
    kiwi_float::kiwi_float::KiwiFloat,
    rfloat::{
        errors::{KiwiFloatResult, RFloatErrors},
        rfstruct::RFloat,
    },
};

impl RFloat {
    pub fn new(
        value: f64,
        total_digits: usize,
        mut decimal_places: usize,
    ) -> KiwiFloatResult<Self> {
        if value.is_infinite() {
            return Err(RFloatErrors::ValueIsInfinity);
        } else if value.is_nan() {
            return Err(RFloatErrors::ValueIsNan);
        }
        let stringed_value = format!("{:.1$}", value, total_digits);
        let (before, after) = stringed_value
            .split_once(".")
            .ok_or(RFloatErrors::UnknownError)?;

        if after.len() < decimal_places {
            decimal_places = after.len()
        }

        let f64_slice = &stringed_value[..(decimal_places + before.len() + 1)];

        let treated_float: f64 =
            f64_slice
                .parse::<f64>()
                .map_err(|_| RFloatErrors::ParseError {
                    thing: f64_slice.to_string(),
                    target: "f64".into(),
                })?;

        Ok((Self {
            value: treated_float,
            digits: decimal_places,
        }))
    }
}

impl std::fmt::Display for RFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Default for RFloat {
    fn default() -> Self {
        Self {
            value: 0.0,
            digits: 30,
        }
    }
}

impl PartialEq for RFloat {
    fn eq(&self, other: &Self) -> bool {
        self.value.to_bits() == other.value.to_bits()
    }
}

impl Eq for RFloat {}

impl Hash for RFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.to_bits().hash(state);
    }
}

impl TryFrom<KiwiFloat> for RFloat {
    type Error = RFloatErrors;
    fn try_from(value: KiwiFloat) -> Result<Self, Self::Error> {
        let formatted: &str = &format!("{}.{}", value.base, value.fractional);

        let parsed = formatted
            .parse::<f64>()
            .map_err(|_| RFloatErrors::ParseError {
                thing: formatted.into(),
                target: "f64".into(),
            })?;

        Ok(Self {
            value: parsed,
            digits: value.decimal_places,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kiwi_float() {
        let joe = RFloat::new(69.6969, 69, 3).unwrap();

        println!("{}", joe.to_string())
    }
}
