//!
//! A crate dedicated to breaking Rust's safety guarantees, written 100% using safe rust!
//!
//! Completed features:
//! * Dereference raw pointers
//! * Call unsafe functions
//!
//! TODO:
//! * Implement unsafe traits
//! * Mutate statics (including external ones)
//! * Access fields of unions
//!
//! The rules are simple: Never create any `unsafe` blocks or functions (except for testing)
//!

pub mod bytes;
pub mod call;
pub mod death_functions;
pub mod extend_lifetime;
pub mod pointer;
pub mod transmute;

use crate::extend_lifetime::*;
use death_functions::death_by_random;

fn main() {
    death_by_random()
}
