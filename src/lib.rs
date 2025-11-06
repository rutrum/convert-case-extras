//! Extra utilities for [`convert_case`].
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

#[cfg(feature = "random")]
use rand::prelude::*;

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
    /// use convert_case_extras::pattern;
    ///
    /// assert_eq!(
    ///     pattern::TOGGLE.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["cASE", "cONVERSION", "lIBRARY"],
    /// );
    /// ```
    pub const TOGGLE: Pattern =
        Pattern::Custom(|words| words.iter().map(|word| toggle_word(word)).collect());

    /// Makes each letter of each word alternate between lowercase and uppercase.
    ///
    /// It alternates across words,
    /// which means the last letter of one word and the first letter of the
    /// next will not be the same letter casing.
    /// ```
    /// use convert_case_extras::pattern;
    ///
    /// assert_eq!(
    ///     pattern::ALTERNATING.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["cAsE", "cOnVeRsIoN", "lIbRaRy"],
    /// );
    /// assert_eq!(
    ///     pattern::ALTERNATING.mutate(&["Another", "Example"]),
    ///     vec!["aNoThEr", "ExAmPlE"],
    /// );
    /// ```
    pub const ALTERNATING: Pattern = Pattern::Custom(|words| {
        let mut upper = false;
        words
            .iter()
            .map(|word| {
                word.chars()
                    .map(|letter| {
                        if letter.is_uppercase() || letter.is_lowercase() {
                            if upper {
                                upper = false;
                                letter.to_uppercase().to_string()
                            } else {
                                upper = true;
                                letter.to_lowercase().to_string()
                            }
                        } else {
                            letter.to_string()
                        }
                    })
                    .collect()
            })
            .collect()
    });

    // #[doc(cfg(feature = "random"))]
    /// Lowercases or uppercases each letter uniformly randomly.
    ///
    /// This uses the `rand` crate and is only available with the "random" feature.
    /// ```
    /// # #[cfg(any(doc, feature = "random"))]
    /// use convert_case_extras::pattern;
    /// pattern::RANDOM.mutate(&["Case", "CONVERSION", "library"]);
    /// // "casE", "coNVeRSiOn", "lIBraRY"
    /// ```
    #[cfg(feature = "random")]
    pub const RANDOM: Pattern = Pattern::Custom(|words| {
        let mut rng = rand::thread_rng();
        words
            .iter()
            .map(|word| {
                word.chars()
                    .map(|letter| {
                        if rng.gen::<f32>() > 0.5 {
                            letter.to_uppercase().to_string()
                        } else {
                            letter.to_lowercase().to_string()
                        }
                    })
                    .collect()
            })
            .collect()
    });
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
    /// use convert_case::Casing;
    /// use convert_case_extras::case;
    /// assert_eq!("My variable NAME".to_case(case::TOGGLE), "mY vARIABLE nAME");
    /// ```
    pub const TOGGLE: Case = Case::Custom {
        boundaries: &[Boundary::Space],
        pattern: pattern::TOGGLE,
        delim: " ",
    };

    /// Alternating case strings are delimited by spaces.  Characters alternate between uppercase
    /// and lowercase.
    /// * Boundaries: [Space](Boundary::Space)
    /// * Pattern: [Alternating](Pattern::Alternating)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::Casing;
    /// use convert_case_extras::case;
    /// assert_eq!("My variable NAME".to_case(case::ALTERNATING), "mY vArIaBlE nAmE");
    /// ```
    pub const ALTERNATING: Case = Case::Custom {
        boundaries: &[Boundary::Space],
        pattern: pattern::ALTERNATING,
        delim: " ",
    };

    /// Random case strings are delimited by spaces and characters are
    /// randomly upper case or lower case.
    ///
    /// This uses the `rand` crate
    /// and is only available with the "random" feature.
    /// * Boundaries: [Space](Boundary::Space)
    /// * Pattern: [Random](pattern::RANDOM)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::Casing;
    /// use convert_case_extras::case;
    /// "My variable NAME".to_case(case::RANDOM);
    /// // "My vaRIAbLE nAme"
    /// ```
    #[cfg(any(doc, feature = "random"))]
    #[cfg(feature = "random")]
    pub const RANDOM: Case = Case::Custom {
        boundaries: &[Boundary::Space],
        pattern: pattern::RANDOM,
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
