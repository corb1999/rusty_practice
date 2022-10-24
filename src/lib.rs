//! # rusty_practice
//! 
//! `rusty_practice` is a non-project, just for goofing and learning rust on my own
//! 
//! there is nothing interesting going here. look away
//! 
// lib.rs file code below...




// in lib.rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = rusty_practice::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
