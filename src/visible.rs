use crate::{consts, error::SudokuError, solver::place_and_propagate, sudoku::Sudoku};

pub(crate) fn place_all_visible_singles(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    get_placements(sudoku)
        .into_iter()
        .try_for_each(|(idx, digit)| {
            if sudoku.digits[idx] == 0 {
                place_and_propagate(sudoku, idx, digit)
            } else {
                Ok(())
            }
        })
}

pub(crate) fn check_all_visible_doubles(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    check_visible_doubles_rows(sudoku)?;
    check_visible_doubles_cols(sudoku)?;
    check_visible_doubles_cells(sudoku)
}

fn get_placements(sudoku: &Sudoku) -> Vec<(usize, consts::BitWidth)> {
    sudoku
        .bitboard
        .iter()
        .enumerate()
        .filter(|(_, &bitboard)| bitboard.count_ones() == 1)
        .map(|(idx, bitboard)| (idx, bitboard.trailing_zeros() as consts::BitWidth))
        .collect()
}

fn check_visible_doubles_rows(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    check_visible_doubles(sudoku, &consts::SAME_ROW)
}

fn check_visible_doubles_cols(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    check_visible_doubles(sudoku, &consts::SAME_COL)
}

fn check_visible_doubles_cells(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    check_visible_doubles(sudoku, &consts::SAME_CELL)
}

fn check_visible_doubles(
    sudoku: &mut Sudoku,
    neighbor_arr: &[[usize; 8]; consts::SIZE],
) -> Result<(), SudokuError> {
    for (idx, neighbors) in neighbor_arr.iter().enumerate() {
        if sudoku.bitboard[idx].count_ones() == 2 {
            check_visible_double_pairs(sudoku, idx, neighbors)?;
        }
    }
    Ok(())
}

fn check_visible_double_pairs(
    sudoku: &mut Sudoku,
    idx: usize,
    neighbors: &[usize; 8],
) -> Result<(), SudokuError> {
    for neighbor in neighbors {
        if sudoku.bitboard[idx] == sudoku.bitboard[*neighbor] {
            check_visible_double_pair(sudoku, idx, *neighbor, neighbors)?;
        }
    }
    Ok(())
}

fn check_visible_double_pair(
    sudoku: &mut Sudoku,
    idx: usize,
    neighbor: usize,
    neighbors: &[usize; 8],
) -> Result<(), SudokuError> {
    let mask = consts::MASK ^ sudoku.bitboard[idx];
    neighbors
        .iter()
        .filter(|&n_idx| *n_idx != neighbor)
        .try_for_each(|n_idx| check_visible_double(sudoku, *n_idx, mask))
}

fn check_visible_double(
    sudoku: &mut Sudoku,
    n_idx: usize,
    mask: consts::BitWidth,
) -> Result<(), SudokuError> {
    sudoku.bitboard[n_idx] &= mask;
    check_visible_double_possible(sudoku, n_idx)?;
    if sudoku.bitboard[n_idx].count_ones() == 1 {
        let digit = sudoku.bitboard[n_idx].trailing_zeros() as consts::BitWidth;
        place_and_propagate(sudoku, n_idx, digit)
    } else {
        // TODO: benchmark if it's faster to have this as an else clause
        Ok(())
    }
}

fn check_visible_double_possible(sudoku: &Sudoku, n_idx: usize) -> Result<(), SudokuError> {
    if (sudoku.digits[n_idx] == 0) && (sudoku.bitboard[n_idx] == 0) {
        Err(SudokuError::from(sudoku))
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
    #[case("69387541214563279878219435635742186981695723442936817527451968396874352153128694.")]
    #[case("38741952625976341864152837971628594359463178282397416547239685113584269796815723.")]
    #[case("38261947559473862117642593886394175245726318992185736473859421624517689361938254.")]
    #[case("53641972891782543624873695178156429369537281442398167537914856285269314716425738.")]
    #[case("46831597279582416313269748581945372625718634934697285192476153858324961767153829.")]
    #[case("51342697898735164264298751383164925727951836445627389179813542636479218512586473.")]
    #[case("36954287172189346554876132941295863783761459295623718418532974669347521827418695.")]
    #[case("27451863936194287585963724198237615414528936773615492851876349269742158342389571.")]
    #[case("57342816962491387519857624383164592794523761876218935428736459131975248645689173.")]
    #[case("73521896441836975262945731885762419394187352636219584728354167919473628557698243.")]
    #[case("43851967221647895395732614816275438984593276179368152432186549767429381558914723.")]
    #[case("36579824198123457674215638943681592721947386557862941319734265885496713262358179.")]
    #[case("96781354241375296852864973135219768467943812584152639778436125919628547323597481.")]
    #[case("13847965272531689469482573154163892728395741697624138586759214345916327831278456.")]
    fn test_visible_singles(#[case] input: &str) -> Result<(), SudokuError> {
        let mut sudoku = Sudoku::from_str(input)?;
        let _ = place_all_visible_singles(&mut sudoku);
        assert!(sudoku.is_solved());
        Ok(())
    }

    #[rstest]
    #[case(
        ".................................................................................",
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000"
    )]
    #[case(
        "123456789........................................................................",
        "123456789000000000000000000000000000000000000000000000000000000000000000000000000"
    )]
    #[case(
        "1.34.67.9...................5..............5.....................................",
        "103456709000000000000000000050000000000000050000000000000000000000000000000000000"
    )]
    #[case(
        "1.34.67.9...................5..............5.....................................",
        "103456709000000000000000000050000000000000050000000000000000000000000000000000000"
    )]
    #[case(
        "1.34.67.9...................5..............5............................234..7891",
        "103456709000000000000000000050000000000000050000000000000000000000000000234567891"
    )]
    fn test_visible_doubles(
        #[case] input: &str,
        #[case] expected: &str,
    ) -> Result<(), SudokuError> {
        let mut sudoku = Sudoku::from_str(input)?;
        check_all_visible_doubles(&mut sudoku)?;
        assert_eq!(sudoku.to_string(), expected);
        Ok(())
    }
}
