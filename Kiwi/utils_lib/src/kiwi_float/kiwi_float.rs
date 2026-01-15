#[derive(Clone, Debug)]
pub struct KiwiFloat {
    /// True if the number is negative (kept even when base == 0, so -0.x works)
    pub negative: bool,

    /// Absolute integer part (0..)
    pub base: u128,

    /// Fractional digits ONLY, no sign, no dot.
    /// Invariant: length == decimal_places (unless decimal_places == 0)
    pub fractional: String,

    /// Number of digits after decimal point represented in `fractional`
    pub decimal_places: usize,
}
