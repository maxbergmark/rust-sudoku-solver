use crate::{
    consts,
    error::SudokuError,
    hidden::{place_all_hidden_singles, place_all_hidden_zeroes},
    sudoku::Sudoku,
    visible::{place_all_visible_doubles, place_all_visible_singles},
};

#[inline]
fn check_branch(
    sudoku: &mut Sudoku,
    idx: usize,
    digit: consts::BitWidth,
) -> Result<Sudoku, SudokuError> {
    let mut cloned_board = sudoku.clone();
    place_and_propagate(&mut cloned_board, idx, digit)?;
    solve_recursive(&mut cloned_board).map_err(|error| sudoku.store_stats(error))
}

fn branch_possibilities(sudoku: &mut Sudoku, idx: usize) -> Result<Sudoku, SudokuError> {
    let bitboard = sudoku.bitboard[idx];
    sudoku.guesses += i32::from(bitboard.count_ones() > 1);
    let start = bitboard.trailing_zeros() as consts::BitWidth;
    let end = consts::NUM_BITS - bitboard.leading_zeros() as consts::BitWidth;

    (start..end)
        .filter(|&n| bitboard & (1 << n) > 0)
        .map(|digit| check_branch(sudoku, idx, digit))
        .find_map(Result::ok)
        .ok_or(SudokuError::from(sudoku))
}

fn solve_recursive(sudoku: &mut Sudoku) -> Result<Sudoku, SudokuError> {
    sudoku.num_recursions += 1;
    check_constraints(sudoku)?;
    if sudoku.is_solved() {
        Ok(sudoku.clone())
    } else if let Some(idx) = get_next_idx(sudoku) {
        branch_possibilities(sudoku, idx)
    } else {
        Err(SudokuError::from(sudoku))
    }
}

#[inline]
fn unit_propagate(sudoku: &mut Sudoku, idx: usize) -> Result<(), SudokuError> {
    for neighbor in consts::NEIGHBORS[idx] {
        let digit = sudoku.digits[neighbor];
        let bitboard = sudoku.bitboard[neighbor];
        if digit == 0 && bitboard == 0 {
            return Err(SudokuError::from(sudoku));
        } else if digit == 0 && bitboard.count_ones() == 1 {
            let digit = sudoku.bitboard[neighbor].trailing_zeros() as consts::BitWidth;
            place_and_propagate(sudoku, neighbor, digit)?;
        }
    }
    Ok(())
}

pub(crate) fn place_and_propagate(
    sudoku: &mut Sudoku,
    idx: usize,
    digit: consts::BitWidth,
) -> Result<(), SudokuError> {
    sudoku.place(idx, digit);
    unit_propagate(sudoku, idx)
}

pub fn solve(mut sudoku: Sudoku) -> Result<Sudoku, SudokuError> {
    // heuristic for attempting to solve the puzzle
    place_all_hidden_singles(&mut sudoku)?;
    place_all_visible_singles(&mut sudoku)?;
    if sudoku.is_solved() {
        return Ok(sudoku);
    }
    place_all_visible_doubles(&mut sudoku)?;
    place_all_hidden_singles(&mut sudoku)?;
    place_all_visible_singles(&mut sudoku)?;
    if sudoku.is_solved() {
        return Ok(sudoku);
    }
    solve_recursive(&mut sudoku)
}

fn get_next_idx(sudoku: &Sudoku) -> Option<usize> {
    (0..consts::SIZE)
        .filter(|&i| sudoku.digits[i] == 0)
        .map(|i| (i, sudoku.bitboard[i].count_ones()))
        .min_by_key(|&(_, num_possibilities)| num_possibilities)
        .map(|(idx, _)| idx)
}

#[inline]
fn check_constraints(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    place_all_hidden_zeroes(sudoku)?;
    place_all_hidden_singles(sudoku)?;
    place_all_visible_singles(sudoku)?;
    place_all_visible_doubles(sudoku)
}
