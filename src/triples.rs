use crate::{consts, error::SudokuError, solver::place_and_propagate, sudoku::Sudoku};

#[allow(unused)]
pub(crate) fn check_triples(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    check_triples_rows_or_cols(sudoku, &get_row, &consts::ROWS)?;
    check_triples_rows_or_cols(sudoku, &get_col, &consts::COLS)
}

fn get_row(idx: usize) -> usize {
    idx / consts::WIDTH
}

fn get_col(idx: usize) -> usize {
    idx % consts::WIDTH
}

fn check_triples_rows_or_cols(
    sudoku: &mut Sudoku,
    get_row_or_col: &dyn Fn(usize) -> usize,
    cols_or_rows: &[[usize; 9]; 9],
) -> Result<(), SudokuError> {
    for (cell_idx, cell) in consts::CELLS.iter().enumerate() {
        let mut cache = [0; 3];
        let base_row_idx = get_row_or_col(cell[0]);
        for &idx in cell {
            let row = get_row_or_col(idx);
            cache[row % 3] |= sudoku.bitboard[idx];
        }
        for digit in 1..=9 {
            check_digit(sudoku, digit, &cache, base_row_idx, cell_idx, cols_or_rows)?;
        }
    }
    Ok(())
}

fn check_digit(
    sudoku: &mut Sudoku,
    digit: i32,
    cache: &[usize; 3],
    base_row_idx: usize,
    cell_idx: usize,
    cols_or_rows: &[[usize; 9]; 9],
) -> Result<(), SudokuError> {
    let bitmask = 1 << digit;
    let matching_rows: Vec<usize> = cache
        .iter()
        .enumerate()
        .filter(|(_, &row_bitmask)| row_bitmask & bitmask > 0)
        .map(|(row_idx, _)| row_idx)
        .collect();

    if let [row_idx] = matching_rows.as_slice() {
        check_triple_digits(
            sudoku,
            digit,
            base_row_idx,
            *row_idx,
            cell_idx,
            cols_or_rows,
        )?;
    }
    Ok(())
}

fn check_triple_digits(
    sudoku: &mut Sudoku,
    digit: i32,
    base_row_idx: usize,
    row_idx: usize,
    cell_idx: usize,
    cols_or_rows: &[[usize; 9]; 9],
) -> Result<(), SudokuError> {
    let row_idx = row_idx + base_row_idx;
    let bitmask = 1 << digit;
    for idx in cols_or_rows[row_idx] {
        if consts::CELL_LOOKUP[idx] != cell_idx {
            sudoku.bitboard[idx] &= consts::MASK ^ bitmask;
            place_triple_digit(sudoku, idx)?;
        }
    }
    Ok(())
}

fn place_triple_digit(sudoku: &mut Sudoku, idx: usize) -> Result<(), SudokuError> {
    if sudoku.digits[idx] == 0 && sudoku.bitboard[idx] == 0 {
        Err(SudokuError::from(sudoku))
    } else if sudoku.bitboard[idx].count_ones() == 1 {
        let digit = sudoku.bitboard[idx].trailing_zeros() as consts::BitWidth;
        place_and_propagate(sudoku, idx, digit)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".........12345...745619..........................................................",
        "000000000123458007456197000000000000000000000000000000000000000000000000000000000"
    )]
    fn test_place(#[case] input: &str, #[case] expected: &str) -> Result<(), SudokuError> {
        let mut sudoku = Sudoku::from_str(input)?;
        println!("{}", debug::pretty_print_alternatives(&sudoku).unwrap());

        check_triples(&mut sudoku)?;
        hidden::place_all_hidden_singles(&mut sudoku)?;

        assert_eq!(sudoku.to_string(), expected);
        Ok(())
    }

    #[test]
    fn test_manual() {
        let input =
            "057000300300801000081703000600030090020070000800000000400607000000000207700000050";
        let mut sudoku = Sudoku::from_str(input).unwrap();
        check_triples(&mut sudoku).unwrap();
        assert_eq!(sudoku.bitboard[58] & (1 << 5), 0);
        hidden::place_all_hidden_singles(&mut sudoku).unwrap();
        assert_eq!(5, sudoku.digits[56]);
    }

    #[test]
    fn test_manual_2() {
        let input =
            "000000036030000052200000000000067820700000400000053000000706210060900000001000000";
        let mut sudoku = Sudoku::from_str(input).unwrap();
        assert_eq!((1 << 6) | (1 << 9), sudoku.bitboard[43]);
        check_triples(&mut sudoku).unwrap();
        assert_eq!(0, sudoku.bitboard[43]);
    }
}
