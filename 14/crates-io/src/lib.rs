//! Crates-io
//!
//! `crates-io` is an example crate to explore cargo and crates.io functionalities.

/// Adds one to the number given
///
/// # Example
/// ```
/// let arg = 5;
/// let answer = crates_io::add_one(arg);
///
/// assert_eq!(answer, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    // normal comment
    x + 1
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        // ANCHOR_END: here
        SecondaryColor::Orange
        // ANCHOR: here
    }
}
