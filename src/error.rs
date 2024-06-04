use crate::sudoku::Sudoku;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum SudokuError {
    SolveError,
    ParseError,
    IndexError,
    NoSolution { num_recursions: i32, guesses: i32 },
}

impl From<&Sudoku> for SudokuError {
    fn from(sudoku: &Sudoku) -> Self {
        SudokuError::NoSolution {
            num_recursions: sudoku.num_recursions,
            guesses: sudoku.guesses,
        }
    }
}

impl From<&mut Sudoku> for SudokuError {
    fn from(sudoku: &mut Sudoku) -> Self {
        SudokuError::NoSolution {
            num_recursions: sudoku.num_recursions,
            guesses: sudoku.guesses,
        }
    }
}
