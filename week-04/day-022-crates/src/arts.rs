//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    #[derive(Debug,PartialEq)]
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(Copy, Clone)]
    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::arts::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        let palette = vec![
            (PrimaryColor::Red, PrimaryColor::Yellow, SecondaryColor::Orange),
            (PrimaryColor::Red, PrimaryColor::Blue, SecondaryColor::Purple),
            (PrimaryColor::Yellow, PrimaryColor::Blue, SecondaryColor::Green)
        ];
        if let Some(item) = palette.iter().find(|p| (p.0 == c1 && p.1 == c2) || (p.0 == c2 && p.1 == c1)) {
            item.2
        } else {
            panic!("No combination found for {:?} {:?}", c1, c2);
        }
    }
}