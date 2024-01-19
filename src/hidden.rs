use crate::consts;
use crate::error::SudokuError;
use crate::solver::place_and_propagate;
use crate::Sudoku;

fn place_hidden_single(
    sudoku: &mut Sudoku,
    idx: usize,
    neighbors: &[usize; 8],
) -> Result<(), SudokuError> {
    let mask = neighbors
        .iter()
        .map(|&i| sudoku.bitboard[i])
        .reduce(|a, b| a | b)
        .ok_or(SudokuError::IndexError)?;

    let bitboard = sudoku.bitboard.get(idx).ok_or(SudokuError::IndexError)?;
    let value = (mask ^ consts::MASK) & bitboard;

    if value.count_ones() == 1 {
        let digit = value.trailing_zeros() as consts::BitWidth;
        place_and_propagate(sudoku, idx, digit)?;
    }
    Ok(())
}

fn place_hidden_singles(
    sudoku: &mut Sudoku,
    neighbor_arr: &[[usize; 8]; consts::SIZE],
) -> Result<(), SudokuError> {
    neighbor_arr
        .iter()
        .enumerate()
        .try_for_each(|(idx, neighbors)| place_hidden_single(sudoku, idx, neighbors))
}

fn place_hidden_singles_rows(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    place_hidden_singles(sudoku, &consts::SAME_ROW)
}

fn place_hidden_singles_cols(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    place_hidden_singles(sudoku, &consts::SAME_COL)
}

fn place_hidden_singles_cells(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    place_hidden_singles(sudoku, &consts::SAME_CELL)
}

pub(crate) fn place_all_hidden_singles(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    place_hidden_singles_rows(sudoku)?;
    place_hidden_singles_cols(sudoku)?;
    place_hidden_singles_cells(sudoku)
}

#[inline]
fn get_mask(sudoku: &Sudoku, neighbors: &[usize; consts::WIDTH]) -> consts::BitWidth {
    neighbors
        .iter()
        .map(|&i| (sudoku.bitboard[i], sudoku.digits[i]))
        .map(|(bitboard, digit)| bitboard | (1 << digit))
        .reduce(|a, b| a | b)
        .unwrap_or(0)
}

#[inline]
fn validate_mask(sudoku: &Sudoku, neighbors: &[usize; consts::WIDTH]) -> Result<(), SudokuError> {
    if get_mask(sudoku, neighbors) < consts::MASK {
        Err(SudokuError::from(sudoku))
    } else {
        Ok(())
    }
}

fn validate_hidden_zeroes(
    sudoku: &Sudoku,
    neighbor_arr: &[[usize; consts::WIDTH]; consts::WIDTH],
) -> Result<(), SudokuError> {
    neighbor_arr
        .iter()
        .try_for_each(|neighbors| validate_mask(sudoku, neighbors))
}

fn place_hidden_zeroes_rows(sudoku: &Sudoku) -> Result<(), SudokuError> {
    validate_hidden_zeroes(sudoku, &consts::ROWS)
}

fn place_hidden_zeroes_cols(sudoku: &Sudoku) -> Result<(), SudokuError> {
    validate_hidden_zeroes(sudoku, &consts::COLS)
}

fn place_hidden_zeroes_cells(sudoku: &Sudoku) -> Result<(), SudokuError> {
    validate_hidden_zeroes(sudoku, &consts::CELLS)
}

pub(crate) fn place_all_hidden_zeroes(sudoku: &Sudoku) -> Result<(), SudokuError> {
    place_hidden_zeroes_rows(sudoku)?;
    place_hidden_zeroes_cols(sudoku)?;
    place_hidden_zeroes_cells(sudoku)
}
