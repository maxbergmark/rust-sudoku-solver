mod consts;
mod debug;
mod error;
mod hidden;
pub mod solver;
mod sudoku;
mod triples;
mod visible;

pub use crate::error::SudokuError;
pub use crate::hidden::place_all_hidden_singles;
pub use crate::solver::{check_constraints, solve};
pub use crate::sudoku::Sudoku;
pub use crate::visible::{check_all_visible_doubles, place_all_visible_singles};
