//! # First Crate
//! `vgebrev_first_crate` is some less than useful code from the Rust book
//! to help me learn about crates.

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = vgebrev_first_crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod arts;