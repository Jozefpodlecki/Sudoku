use crate::bits::Digit;
use crate::cell::Cell;

use super::cover::DIGITS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub cell: Cell,
    pub digit: Digit,
}

impl Placement {
    pub(super) fn from_row(row: u32) -> Self {
        let row_index = row as usize;
        let cell_index = row_index / DIGITS;
        let digit_index = row_index % DIGITS;
        Self {
            cell: Cell(cell_index as u8),
            digit: Digit::new((digit_index + 1) as u8).unwrap(),
        }
    }

}