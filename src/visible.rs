use crate::{consts, sudoku::{Sudoku, SudokuError}};


fn get_placements(sudoku: &Sudoku) -> Vec<(usize, consts::BitWidth)> {
    sudoku.bitboard.iter().enumerate()
        .filter(|(_, &n)| n.count_ones() == 1)
        .map(|(i, n)| (i, n.trailing_zeros() as consts::BitWidth))
        .collect()
        
}

pub(crate) fn place_all_visible_singles(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    for (i, n) in get_placements(sudoku) {
        if sudoku.digits[i] == 0 {
            sudoku.place_and_propagate(i, n)?;
        }
    }
    Ok(())
}

fn check_visible_double_possible(sudoku: &Sudoku, n_idx: usize) -> Result<(), SudokuError> {
    if (sudoku.digits[n_idx] == 0) && (sudoku.bitboard[n_idx] == 0) {
        Err(SudokuError::from(sudoku))
    } else {
        Ok(())
    }

}

fn place_visible_double(sudoku: &mut Sudoku, n_idx: usize, mask: consts::BitWidth) -> Result<(), SudokuError> {
    sudoku.bitboard[n_idx] &= mask;
    check_visible_double_possible(sudoku, n_idx)?;
    if sudoku.bitboard[n_idx].count_ones() == 1 {
        let digit = sudoku.bitboard[n_idx].trailing_zeros() as consts::BitWidth;
        sudoku.place_and_propagate(n_idx, digit)?;
    } 
    Ok(())
}

fn check_visible_double_pair(sudoku: &mut Sudoku, idx: usize, neighbor: usize, neighbors: &[usize; 8]) -> Result<(), SudokuError> {
    let mask = consts::MASK ^ sudoku.bitboard[idx];
    for &n_idx in neighbors {
        if n_idx != neighbor {
            place_visible_double(sudoku, n_idx, mask)?;
        }
    }
    Ok(())
}

fn place_visible_double_pair(sudoku: &mut Sudoku, idx: usize, neighbors: &[usize; 8]) -> Result<(), SudokuError> {
    for neighbor in neighbors {
        if sudoku.bitboard[idx] == sudoku.bitboard[*neighbor] {
            check_visible_double_pair(sudoku, idx, *neighbor, neighbors)?;
        }
    }
    Ok(())
}

fn place_visible_doubles(sudoku: &mut Sudoku, neighbor_arr: &[[usize; 8]; consts::SIZE]) -> Result<(), SudokuError> {
    for (idx, neighbors) in neighbor_arr.iter().enumerate() {
        if sudoku.bitboard[idx].count_ones() == 2 {
            place_visible_double_pair(sudoku, idx, neighbors)?;
        }
    }
    Ok(())
}

fn place_visible_doubles_rows(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    place_visible_doubles(sudoku, &consts::SAME_ROW)
}

fn place_visible_doubles_cols(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    place_visible_doubles(sudoku, &consts::SAME_COL)
}

fn place_visible_doubles_cells(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    place_visible_doubles(sudoku, &consts::SAME_CELL)
}

pub(crate) fn place_all_visible_doubles(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    place_visible_doubles_rows(sudoku)?;
    place_visible_doubles_cols(sudoku)?;
    place_visible_doubles_cells(sudoku)
}

