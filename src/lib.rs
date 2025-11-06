//! Extra utilities for `convert_case`.
//!
//! ```
//! use convert_case::Casing;
//! use convert_case_extras::case;
//!
//! assert_eq!(
//!     "toggle_case_word".to_case(case::TOGGLE),
//!     "tOGGLE cASE wORD",
//! )
//! ```

use convert_case::{Boundary, Case, Pattern};

pub mod pattern {
    use super::*;

    /// Applies toggle pattern to a single word using graphemes.
    fn toggle_word(word: &str) -> String {
        let mut chars = word.chars();

        if let Some(c) = chars.next() {
            [c.to_lowercase().collect(), chars.as_str().to_uppercase()].concat()
        } else {
            String::new()
        }
    }

    /// Makes the first letter of each word lowercase
    /// and the remaining letters of each word uppercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Toggle.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["cASE", "cONVERSION", "lIBRARY"],
    /// );
    /// ```
    pub const TOGGLE: Pattern =
        Pattern::Custom(|words| words.iter().map(|word| toggle_word(word)).collect());
}

pub mod case {
    use super::*;

    /// Toggle case strings are delimited by spaces.  All characters are uppercase except
    /// for the leading character of each word, which is lowercase.
    /// * Boundaries: [Space](`Boundary::Space`)
    /// * Pattern: [Toggle](`pattern::TOGGLE`)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(toggle, "My variable NAME"), "mY vARIABLE nAME");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Toggle), "mY vARIABLE nAME");
    /// ```
    pub const TOGGLE: Case = Case::Custom {
        boundaries: &[Boundary::Space],
        pattern: pattern::TOGGLE,
        delim: " ",
    };
}

#[cfg(test)]
mod test {
    use super::*;

    use convert_case::Casing;

    #[test]
    fn toggle_case() {
        assert_eq!("test_toggle".to_case(case::TOGGLE), "tEST tOGGLE");
    }
}
