use std::hash::Hash;

use crate::{
    kiwi_float::{
        errors::{KiwiFloatError, KiwiFloatResult},
        kiwi_float::KiwiFloat,
    },
    rfloat::rfstruct::RFloat,
    traits::{FlipSign, Round},
};

impl KiwiFloat {
    /// Build from f64 by formatting to a given precision (digits after decimal),
    /// then storing base + fractional DIGITS (not as an integer).
    ///
    /// NOTE:
    /// - `total_digits` is treated as a *formatting precision* (digits after decimal)
    /// - `places_after_decimal` is the *desired stored precision*
    /// We format with max(total_digits, places_after_decimal), then round down to places_after_decimal.
    pub fn new(
        float: f64,
        total_digits: usize,
        places_after_decimal: usize,
    ) -> KiwiFloatResult<Self> {
        if float.is_infinite() {
            return Err(KiwiFloatError::ValueIsInfinity);
        }
        if float.is_nan() {
            return Err(KiwiFloatError::ValueIsNan);
        }

        let format_precision = total_digits.max(places_after_decimal);

        // Force fixed-point formatting (no exponent).
        // This DOES still reflect f64 reality, but we keep it safe & deterministic.
        let formatted = format!("{:.*}", format_precision, float);

        Self::from_formatted_decimal(&formatted, places_after_decimal)
    }

    /// Convert to f64 (lossy). Fine for output, DO NOT use for Eq/Hash.
    pub fn to_float64(&self) -> f64 {
        let s = self.to_string();
        s.parse::<f64>().unwrap_or_default()
    }

    /// Internal: parse "-12.3400" into parts and store exactly `target_decimals` decimals (rounded).
    fn from_formatted_decimal(s: &str, target_decimals: usize) -> KiwiFloatResult<Self> {
        let mut negative = false;
        let mut rest = s;

        if let Some(stripped) = rest.strip_prefix('-') {
            negative = true;
            rest = stripped;
        } else if let Some(stripped) = rest.strip_prefix('+') {
            rest = stripped;
        }

        let (base_part, frac_part) = match rest.split_once('.') {
            Some((a, b)) => (a, b),
            None => (rest, ""),
        };

        let base: u128 = base_part.parse().map_err(|_| KiwiFloatError::ParseError {
            thing: base_part.into(),
            target: "u128".into(),
        })?;

        // Keep only digits in frac_part (format! should already guarantee digits, but we’re defensive)
        let mut fractional: String = frac_part.chars().filter(|c| c.is_ascii_digit()).collect();

        let mut decimal_places = fractional.len();

        // Normalize fractional to be exactly decimal_places
        // (it already is, but we keep the invariant clear)
        if decimal_places == 0 {
            fractional.clear();
        }

        let mut out = Self {
            negative,
            base,
            fractional,
            decimal_places,
        };

        // Round to target decimals (also handles padding if target > current)
        out.round_to(target_decimals);

        // If it’s -0.0000... normalize sign away
        out.normalize_zero_sign();

        Ok(out)
    }

    fn is_zero_value(&self) -> bool {
        if self.base != 0 {
            return false;
        }
        self.fractional.chars().all(|c| c == '0')
    }

    fn normalize_zero_sign(&mut self) {
        if self.is_zero_value() {
            self.negative = false;
        }
    }

    /// Return a normalized tuple used for Eq/Hash:
    /// - removes trailing zeros from fractional
    /// - normalizes -0.xxx to +0.xxx if it’s actually zero
    fn normalized_parts(&self) -> (bool, u128, String) {
        let mut neg = self.negative;
        let base = self.base;

        let mut frac = self.fractional.clone();
        while frac.ends_with('0') {
            frac.pop();
        }

        // If fractional becomes empty, treat it as "0" for stable hashing/comparison
        if frac.is_empty() {
            frac.push('0');
        }

        // Normalize negative zero
        if base == 0 && frac == "0" {
            neg = false;
        }

        (neg, base, frac)
    }
}

// ----- Impl Trait Block -----
impl std::fmt::Display for KiwiFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_zero = self.is_zero_value();
        if self.decimal_places == 0 {
            if self.negative && !is_zero {
                return write!(f, "-{}", self.base);
            }
            return write!(f, "{}", self.base);
        }

        let sign = if self.negative && !is_zero { "-" } else { "" };
        write!(f, "{}{}.{}", sign, self.base, self.fractional)
    }
}

impl PartialEq for KiwiFloat {
    fn eq(&self, other: &Self) -> bool {
        self.normalized_parts() == other.normalized_parts()
    }
}

impl Eq for KiwiFloat {}

impl Hash for KiwiFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.normalized_parts().hash(state);
    }
}

impl FlipSign for KiwiFloat {
    fn flip_sign(&mut self) -> &mut Self {
        if self.is_zero_value() {
            self.negative = false;
            return self;
        }
        self.negative = !self.negative;
        self
    }
}

impl Round for KiwiFloat {
    /// Round to exactly `digits` digits after decimal.
    /// - If digits > current, pads zeros.
    /// - If digits < current, rounds half-up using digit-carry (safe).
    fn round_to(&mut self, digits: usize) -> &mut Self {
        // If we currently have fewer decimals, pad with zeros
        if digits > self.decimal_places {
            if self.decimal_places == 0 {
                self.fractional = "0".repeat(digits);
            } else {
                self.fractional
                    .push_str(&"0".repeat(digits - self.decimal_places));
            }
            self.decimal_places = digits;
            self.normalize_zero_sign();
            return self;
        }

        // If same, nothing
        if digits == self.decimal_places {
            self.normalize_zero_sign();
            return self;
        }

        // digits < current: we need rounding
        // If digits == 0, we round based on first fractional digit
        let round_digit = self
            .fractional
            .as_bytes()
            .get(digits)
            .copied()
            .unwrap_or(b'0');

        let needs_round_up = round_digit >= b'5';

        // Truncate fractional to the kept digits
        let mut kept = if digits == 0 {
            String::new()
        } else {
            self.fractional.chars().take(digits).collect()
        };

        if needs_round_up {
            if digits == 0 {
                // Carry straight into base
                self.base = self.base.saturating_add(1);
            } else {
                // Add 1 with carry from right to left in the kept fractional string
                let mut bytes: Vec<u8> = kept.into_bytes();
                let mut i = bytes.len();

                while i > 0 {
                    i -= 1;
                    if bytes[i] < b'9' {
                        bytes[i] += 1;
                        // done carrying
                        kept = String::from_utf8(bytes.clone()).unwrap_or_default();
                        break;
                    } else {
                        bytes[i] = b'0';
                        // keep carrying left
                    }
                }

                // If we carried past the leftmost digit, increment base
                if bytes.iter().all(|&c| c == b'0') {
                    // Example: 1.999 -> 2.000
                    self.base = self.base.saturating_add(1);
                }

                kept = String::from_utf8(bytes).unwrap_or_default();
            }
        }

        // Set new fractional
        if digits == 0 {
            self.fractional.clear();
        } else {
            // Ensure exact length (pad zeros if something went weird)
            if kept.len() < digits {
                kept.push_str(&"0".repeat(digits - kept.len()));
            }
            self.fractional = kept;
        }

        self.decimal_places = digits;
        self.normalize_zero_sign();
        self
    }
}

// ----- Other Trait Stuff -----

impl TryFrom<RFloat> for KiwiFloat {
    type Error = KiwiFloatError;

    fn try_from(value: RFloat) -> Result<Self, Self::Error> {
        // RFloat says: value.value with value.digits decimals
        let formatted = format!("{:.*}", value.digits, value.value);
        KiwiFloat::from_formatted_decimal(&formatted, value.digits)
    }
}

impl TryFrom<f64> for KiwiFloat {
    type Error = KiwiFloatError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_nan() {
            return Err(KiwiFloatError::ValueIsNan);
        }
        if value.is_infinite() {
            return Err(KiwiFloatError::ValueIsInfinity);
        }

        // POLICY: f64 → KiwiFloat uses 12 decimal places by default
        const DEFAULT_DECIMALS: usize = 12;

        // We format at a bit higher precision than we store to make rounding cleaner,
        const FORMAT_DECIMALS: usize = 18;

        KiwiFloat::new(value, FORMAT_DECIMALS, DEFAULT_DECIMALS)
    }
}

// ----- Tests -----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joe() {
        let flaot = -69.6969;
        let mut kiwi = KiwiFloat::new(flaot, 69, 30).unwrap();
        println!("{}", kiwi.to_string())
    }
}
