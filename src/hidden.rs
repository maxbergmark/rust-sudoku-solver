use crate::consts;
use crate::error::{Error, Result};
use crate::solver::place_and_propagate;
use crate::Sudoku;

/// Place all hidden singles in the Sudoku.
///
/// # Errors
///
/// Returns an error if the Sudoku is invalid.
pub fn place_all_hidden_singles(sudoku: &mut Sudoku) -> Result<()> {
    place_hidden_singles_rows(sudoku)?;
    place_hidden_singles_cols(sudoku)?;
    place_hidden_singles_cells(sudoku)
}

fn place_hidden_singles_rows(sudoku: &mut Sudoku) -> Result<()> {
    place_hidden_singles(sudoku, &consts::SAME_ROW)
}

fn place_hidden_singles_cols(sudoku: &mut Sudoku) -> Result<()> {
    place_hidden_singles(sudoku, &consts::SAME_COL)
}

fn place_hidden_singles_cells(sudoku: &mut Sudoku) -> Result<()> {
    place_hidden_singles(sudoku, &consts::SAME_CELL)
}

fn place_hidden_singles(
    sudoku: &mut Sudoku,
    neighbor_arr: &[[usize; 8]; consts::SIZE],
) -> Result<()> {
    neighbor_arr
        .iter()
        .enumerate()
        .try_for_each(|(idx, neighbors)| place_hidden_single(sudoku, idx, neighbors))
}

fn place_hidden_single(sudoku: &mut Sudoku, idx: usize, neighbors: &[usize; 8]) -> Result<()> {
    let mask = get_hidden_singles_mask(sudoku, neighbors);
    let bitboard = sudoku.bitboard[idx];
    let value = (mask ^ consts::MASK) & bitboard;

    if value.count_ones() == 1 {
        let digit = value.trailing_zeros() as consts::BitWidth;
        place_and_propagate(sudoku, idx, digit)
    } else {
        Ok(())
    }
}

#[inline]
fn get_hidden_singles_mask(sudoku: &Sudoku, neighbors: &[usize; 8]) -> consts::BitWidth {
    neighbors
        .iter()
        .map(|&i| sudoku.bitboard[i])
        .reduce(|a, b| a | b)
        .unwrap_or(0)
    // .ok_or(SudokuError::IndexError)
}

pub fn check_all_hidden_zeroes(sudoku: &Sudoku) -> Result<()> {
    check_hidden_zeroes_rows(sudoku)?;
    check_hidden_zeroes_cols(sudoku)?;
    check_hidden_zeroes_cells(sudoku)
}

fn check_hidden_zeroes_rows(sudoku: &Sudoku) -> Result<()> {
    validate_hidden_zeroes(sudoku, &consts::ROWS)
}

fn check_hidden_zeroes_cols(sudoku: &Sudoku) -> Result<()> {
    validate_hidden_zeroes(sudoku, &consts::COLS)
}

fn check_hidden_zeroes_cells(sudoku: &Sudoku) -> Result<()> {
    validate_hidden_zeroes(sudoku, &consts::CELLS)
}

fn validate_hidden_zeroes(
    sudoku: &Sudoku,
    neighbor_arr: &[[usize; consts::WIDTH]; consts::WIDTH],
) -> Result<()> {
    neighbor_arr
        .iter()
        .try_for_each(|neighbors| validate_mask(sudoku, neighbors))
}

#[inline]
fn validate_mask(sudoku: &Sudoku, neighbors: &[usize; consts::WIDTH]) -> Result<()> {
    if get_hidden_zeroes_mask(sudoku, neighbors) < consts::MASK {
        Err(Error::from(sudoku))
    } else {
        Ok(())
    }
}

#[inline]
fn get_hidden_zeroes_mask(sudoku: &Sudoku, neighbors: &[usize; consts::WIDTH]) -> consts::BitWidth {
    neighbors
        .iter()
        .map(|&i| (sudoku.bitboard[i], sudoku.digits[i]))
        .map(|(bitboard, digit)| bitboard | (1 << digit))
        .reduce(|a, b| a | b)
        .unwrap_or(0)
}

#[cfg(test)]
#[allow(clippy::panic_in_result_fn)]
mod tests {
    use super::super::*;
    use super::*;
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[case(
        ".................................................................................",
        Ok(())
    )]
    #[case(
        "123456789........................................................................",
        Ok(())
    )]
    #[case(
        ".234567891.......................................................................",
        Err(Error::SolveError)
    )]
    #[case(
        "64931528713248769558729641387312956495176432826453817939685274141867395272594183.",
        Ok(())
    )]
    fn test_hidden_zeroes(#[case] input: &str, #[case] expected: Result<()>) -> Result<()> {
        let sudoku = Sudoku::from_str(input)?;

        let result = check_all_hidden_zeroes(&sudoku);
        match expected {
            Ok(()) => assert!(result.is_ok()),
            Err(_) => assert!(result.is_err()),
        }
        Ok(())
    }

    #[rstest]
    #[case(
        ".................................................................................",
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000"
    )]
    #[case(
        "123...789.....................5............................5.....................",
        "123050789000000000000000000000500000000000000000000000000005000000000000000000000"
    )]
    #[case(
        "123...789.....................5........6...................5.....................",
        "123456789000000000000000000000500000000600000000000000000005000000000000000000000"
    )]
    fn test_hidden_singles(#[case] input: &str, #[case] expected: &str) -> Result<()> {
        let mut sudoku = Sudoku::from_str(input)?;
        place_all_hidden_singles(&mut sudoku)?;
        assert_eq!(sudoku.to_string(), expected);
        Ok(())
    }
}
