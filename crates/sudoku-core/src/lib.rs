// #![no_std]
// #![feature(generic_const_exprs, portable_simd)]
#![allow(incomplete_features)]

extern crate alloc;

pub mod bits;
pub mod cell;
pub mod difficulty;
pub mod dlx;
pub mod error;
pub mod generator;
pub mod grid;
pub mod peers;
pub mod rating;
pub mod solver;
pub mod techniques;

pub use bits::{Digit, Mask};
pub use cell::Cell;
pub use difficulty::GameDifficulty;
pub use error::SudokuError;
pub use grid::Grid;