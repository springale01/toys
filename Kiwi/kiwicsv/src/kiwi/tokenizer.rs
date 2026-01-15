use std::borrow::Cow;

use crate::kiwi::errors::{KiwiError, KiwiResult};

#[derive(Debug)]
pub struct KiwiTokenizer<'a> {
    /// The Delimiter that would be used for future operations
    delimiter: &'a str,
    /// Whether to fill in missing values, for example filling something to 7 cells if the most cell is 7
    fill: bool,
    /// Trim or nah
    trim: bool,
}

impl<'a> KiwiTokenizer<'a> {
    // ----- Builder Patterns -----
    /// Loads with Default Settings
    /// * Delim:  ", "
    /// * Fill:  false
    /// * Trim:  true
    pub fn new() -> Self {
        KiwiTokenizer::default()
    }
    /// Changes the Delim of the Tokenizer
    pub fn with_delim(mut self, delim: &'a str) -> Self {
        self.delimiter = delim;

        self
    }
    /// Changes Whether to Fill or not with the Tokenizer
    pub fn with_fill(mut self, fill: bool) -> Self {
        self.fill = fill;

        self
    }
    /// Changes Whether to Trim or not With the Tokenizer
    pub fn with_trim(mut self, trim: bool) -> Self {
        self.trim = trim;

        self
    }
    // ----- Methods -----
    /// Tokenizes a string into a 2D array.
    ///
    /// Takes in parameters and settings declared in the struct KiwiTokenizer
    ///
    /// Returns a 2D vector of `Cow<'a, str>` values. Callers should be aware
    /// that the returned values may be borrowed from the input and may
    /// require dereferencing when used.
    ///
    /// *Please Don't blame me if you have to deref it later on :P*
    pub fn tokenize(&self, content: &'a str) -> KiwiResult<Vec<Vec<Cow<'a, str>>>> {
        if content.is_empty() {
            return Err(KiwiError::NoContent);
        }

        let mut out = content
            .lines()
            .map(|line| {
                if self.trim {
                    line.trim()
                        .split_terminator(self.delimiter)
                        .map(|string| Cow::Borrowed(string))
                        .collect::<Vec<Cow<'a, str>>>()
                } else {
                    line.split_terminator(self.delimiter)
                        .map(|string| Cow::Borrowed(string))
                        .collect::<Vec<Cow<'a, str>>>()
                }
            })
            .collect::<Vec<Vec<Cow<'a, str>>>>();

        // ----- Filling Part! ------
        if self.fill {
            if out.len() == 1 {
                return Ok(out); // You can't fill something that's one line
            }

            let longest = out
                .iter()
                .map(|line| line.len())
                .max()
                .ok_or(KiwiError::NoContent)?;

            for lines in out.iter_mut() {
                let difference = longest - lines.len();

                for i in 0..difference {
                    lines.push(Cow::Borrowed("NaN"));
                }
            }

            return Ok(out);
        }

        return Ok(out);
    }
}

// ----- Impl Traits -----
impl<'a> Default for KiwiTokenizer<'a> {
    fn default() -> Self {
        Self {
            delimiter: ", ",
            fill: false,
            trim: true,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokenizer = KiwiTokenizer {
            delimiter: ", ",
            fill: true,
            trim: true,
        };
        let testing = "Joe, Apples, Me, 12, 99332, -3223, Happy, Twig\n Joe, me, 39, 49, Joebeef\nWhat, is, 39, ijoija, 932";
        let joe = KiwiTokenizer::new()
            .with_delim(", ")
            .with_fill(true)
            .with_trim(true);

        println!("{:?}", joe.tokenize(testing))
    }
}
