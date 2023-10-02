use crate::{consts, sudoku::{Sudoku, SudokuError}};



impl Sudoku {

    #[allow(unused)]
    pub(crate) fn check_all_visible_singles(&mut self) -> Result<(), SudokuError> {
        // loop {
            let mut placements = vec![];
            for (i, &n) in self.bitboard.iter().enumerate() {
                if n.count_ones() == 1 {
                    placements.push((i, n.trailing_zeros() as consts::BitWidth));
                }
            }

            // if placements.is_empty() {
                // break;
            // } else {
                for (i, n) in placements {
                    // println!("from visible singles");
                    if self.digits[i] == 0 {
                        *self = self.place_and_copy(i, n)?;
                    }
                }    
            // }
        // }
        Ok(())
    }

    #[inline]
    fn check_visible_double_pair(&mut self, idx: usize, neighbor: usize, neighbors: &[usize; 8]) -> Result<(), SudokuError> {
        if self.bitboard[idx] == self.bitboard[neighbor] {
            let mask = consts::MASK ^ self.bitboard[idx];
            for n_idx in neighbors {
                if *n_idx != neighbor {
                    self.bitboard[*n_idx] &= mask;
                    if (self.digits[*n_idx] == 0) & (self.bitboard[*n_idx] == 0) {
                        return Err(SudokuError {
                            digit: 0,
                            index: idx,
                            num_recursions: self.num_recursions, 
                            guesses: self.guesses
                        });
                    }
                    if (self.digits[*n_idx] == 0) & (self.bitboard[*n_idx].count_ones() == 1) {
                        // println!("placing immediately from double pair");
                        let digit = self.bitboard[*n_idx].trailing_zeros() as consts::BitWidth;
                        *self = self.place_and_copy(*n_idx, digit)?;
                    } 
                }
            }
        }
        Ok(())
    }

    fn check_visible_doubles(&mut self, neighbor_arr: &[[usize; 8]; consts::SIZE]) -> Result<(), SudokuError> {
        for (idx, neighbors) in neighbor_arr.iter().enumerate() {
            if (self.bitboard[idx] > 0) & (self.bitboard[idx].count_ones() == 2) {
                for neighbor in neighbors {
                    self.check_visible_double_pair(idx, *neighbor, neighbors)?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn check_all_visible_doubles(&mut self) -> Result<(), SudokuError> {
        self.check_visible_doubles(&consts::SAME_ROW)?;
        self.check_visible_doubles(&consts::SAME_COL)?;
        self.check_visible_doubles(&consts::SAME_CELL)
    }

    fn check_visible_n(&mut self, n: usize, neighbor_arr: &[usize; consts::WIDTH]) -> Result<(), SudokuError> {

        use itertools::Itertools;
        for idxs in neighbor_arr.iter().filter(|&i| self.digits[*i] == 0).combinations(n) {
            let mut mask = 0;
            for &i in idxs.iter() {
                mask |= self.bitboard[*i];
            }
            if mask.count_ones() == n as u32 {
                let inverted_mask = consts::MASK ^ mask;
                for idx in neighbor_arr {
                    if !idxs.contains(&idx) {
                        self.bitboard[*idx] &= inverted_mask;
                        if (self.bitboard[*idx] == 0) & (self.digits[*idx] == 0) {
                            return Err(SudokuError {
                                digit: 0,
                                index: *idx,
                                num_recursions: self.num_recursions,
                                guesses: self.guesses
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[allow(unused)]
    pub(crate) fn check_all_visible_n(&mut self, n: usize) -> Result<(), SudokuError> {
        for i in 0..9 {
            self.check_visible_n(n, &consts::ROWS[i])?;
            self.check_visible_n(n, &consts::COLS[i])?;
            self.check_visible_n(n, &consts::CELLS[i])?;
        }
        Ok(())
    }
}