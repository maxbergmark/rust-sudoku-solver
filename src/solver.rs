use crate::{
    consts,
    hidden::{check_all_hidden_zeroes, place_all_hidden_singles},
    sudoku::Sudoku,
    triples::check_triples,
    visible::{check_all_visible_doubles, place_all_visible_singles},
    Error, Result,
};

/// Solves a Sudoku puzzle. Returns the solved Sudoku if successful.
/// If multiple solutions are possible, only one is returned.
///
/// # Errors
///
/// Returns an error if the Sudoku is invalid.
pub fn solve(mut sudoku: Sudoku) -> Result<Sudoku> {
    // heuristic for attempting to solve the puzzle
    place_all_visible_singles(&mut sudoku)?;
    if sudoku.is_solved() {
        return Ok(sudoku);
    }
    place_all_hidden_singles(&mut sudoku)?;
    if sudoku.is_solved() {
        return Ok(sudoku);
    }
    check_triples(&mut sudoku)?;
    check_all_visible_doubles(&mut sudoku)?;
    place_all_hidden_singles(&mut sudoku)?;
    place_all_visible_singles(&mut sudoku)?;
    if sudoku.is_solved() {
        return Ok(sudoku);
    }
    solve_recursive(&mut sudoku)
}

/// Perform all constraint checks on the Sudoku.
///
/// # Errors
///
/// Returns an error if the Sudoku is invalid.
#[inline]
pub fn check_constraints(sudoku: &mut Sudoku) -> Result<()> {
    check_triples(sudoku)?;
    place_all_visible_singles(sudoku)?;
    check_all_hidden_zeroes(sudoku)?;
    place_all_hidden_singles(sudoku)?;
    check_all_visible_doubles(sudoku)
}

fn solve_recursive(sudoku: &mut Sudoku) -> Result<Sudoku> {
    sudoku.num_recursions += 1;
    check_constraints(sudoku)?;
    if sudoku.is_solved() {
        Ok(sudoku.clone())
    } else if let Some(idx) = get_next_idx(sudoku) {
        branch_possibilities(sudoku, idx)
    } else {
        Err(Error::from(sudoku))
    }
}

pub(crate) fn place_and_propagate(
    sudoku: &mut Sudoku,
    idx: usize,
    digit: consts::BitWidth,
) -> Result<()> {
    sudoku.place(idx, digit);
    unit_propagate(sudoku, idx)
}

fn branch_possibilities(sudoku: &mut Sudoku, idx: usize) -> Result<Sudoku> {
    let bitboard = sudoku.bitboard[idx];
    sudoku.guesses += i32::from(bitboard.count_ones() > 1);
    let start = bitboard.trailing_zeros() as consts::BitWidth;
    let end = consts::NUM_BITS - bitboard.leading_zeros() as consts::BitWidth;

    (start..end)
        .filter(|&n| bitboard & (1 << n) > 0)
        .map(|digit| check_branch(sudoku, idx, digit))
        .find_map(Result::ok)
        .ok_or_else(|| Error::from(sudoku))
}

#[inline]
fn check_branch(sudoku: &mut Sudoku, idx: usize, digit: consts::BitWidth) -> Result<Sudoku> {
    let mut cloned_board = sudoku.clone();
    place_and_propagate(&mut cloned_board, idx, digit)?;
    solve_recursive(&mut cloned_board).map_err(|error| sudoku.store_stats(error))
}

#[inline]
fn unit_propagate(sudoku: &mut Sudoku, idx: usize) -> Result<()> {
    for neighbor in consts::NEIGHBORS[idx] {
        let current_digit = sudoku.digits[neighbor];
        let bitboard = sudoku.bitboard[neighbor];
        if current_digit == 0 && bitboard == 0 {
            return Err(Error::from(sudoku));
        } else if current_digit == 0 && bitboard.count_ones() == 1 {
            let digit = sudoku.bitboard[neighbor].trailing_zeros() as consts::BitWidth;
            place_and_propagate(sudoku, neighbor, digit)?;
        }
    }
    Ok(())
}

fn get_next_idx(sudoku: &Sudoku) -> Option<usize> {
    (0..consts::SIZE)
        .filter(|&i| sudoku.digits[i] == 0)
        .map(|i| (i, sudoku.bitboard[i].count_ones()))
        .min_by_key(|&(_, num_possibilities)| num_possibilities)
        .map(|(idx, _)| idx)
}

#[cfg(test)]
#[allow(clippy::panic_in_result_fn)]
mod tests {
    use crate::solver;
    use crate::sudoku::Sudoku;
    use crate::Result;
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[case(
        "000000010400000000020000000000050407008000300001090000300400200050100000000806000",
        "693784512487512936125963874932651487568247391741398625319475268856129743274836159"
    )]
    #[case(
        "000000032040000000900000000302700050000100800600000000070000100080060000000030006",
        "861475932247398615935612748392786451754123869618954327576249183183567294429831576"
    )]
    #[case(
        "........8..3...4...9..2..6.....79.......612...6.5.2.7...8...5...1.....2.4.5.....3",
        "621943758783615492594728361142879635357461289869532174238197546916354827475286913"
    )]
    #[case(
        ".................................................................................",
        "123456789456789123789123456231674895875912364694538217317265948542897631968341572"
    )]
    fn test_sudokus(#[case] input: &str, #[case] expected: &str) -> Result<()> {
        let sudoku = Sudoku::from_str(input)?;
        let solution = solver::solve(sudoku)?;
        assert_eq!(solution.to_string(), expected);
        Ok(())
    }

    #[rstest]
    #[case(
        "057000300000801000001000000600030090020070000800000000400600000000000207000000050",
        "957264381346851972281793645614532798529478136873916524435627819198345267762189453",
        3
    )]
    #[case(
        "000000036030000050200000000000060800700000400000053000000700210060900000001000000",
        "148572936637894152295631748314267895756189423829453671583746219462915387971328564",
        318
    )]
    fn test_manual(
        #[case] input: &str,
        #[case] expected: &str,
        #[case] expected_recursions: i32,
    ) -> Result<()> {
        let sudoku = Sudoku::from_str(input)?;
        let solution = solver::solve(sudoku)?;
        assert_eq!(&solution.to_string(), expected);
        assert_eq!(expected_recursions, solution.num_recursions);

        Ok(())
    }
}
