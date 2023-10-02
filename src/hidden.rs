use crate::Sudoku;
use crate::consts;
use crate::sudoku::SudokuError;

impl Sudoku {

    fn check_hidden_single(&mut self, idx: usize, neighbors: &[usize; 8]) -> Result<(), SudokuError> {

        let mut mask = 0;
        for i in neighbors {
            mask |= self.bitboard[*i];
        }
        mask ^= consts::MASK;

        let value = mask & self.bitboard[idx];
        
        if value.count_ones() == 1 {
            let digit = value.trailing_zeros() as consts::BitWidth;
            *self = self.place_and_copy(idx, digit)?;
        }
        Ok(())
    }

    fn check_hidden_singles(&mut self, neighbor_arr: &[[usize; 8]; consts::SIZE]) -> Result<(), SudokuError> {
        for (idx, neighbors) in neighbor_arr.iter().enumerate() {
            self.check_hidden_single(idx, neighbors)?;
        }
        Ok(())
    }

    pub(crate) fn check_all_hidden_singles(&mut self) -> Result<(), SudokuError> {
        self.check_hidden_singles(&consts::SAME_ROW)?;
        self.check_hidden_singles(&consts::SAME_COL)?;
        self.check_hidden_singles(&consts::SAME_ROW)
    }


    fn check_hidden_zeroes(&mut self, neighbor_arr: &[[usize; consts::WIDTH]; consts::WIDTH]) -> Result<(), SudokuError> {

        for neighbors in neighbor_arr {
            let mut mask = 0;
            for i in neighbors {
                mask |= self.bitboard[*i];
                mask |= 1 << self.digits[*i];
            }

            if mask & consts::MASK != consts::MASK {
                    return Err(SudokuError { 
                        digit: 0,
                        index: 0,
                        num_recursions: self.num_recursions, 
                        guesses: self.guesses 
                    });
            }
        }

        Ok(())
    }

    pub(crate) fn check_all_hidden_zeroes(&mut self) -> Result<(), SudokuError> {
        self.check_hidden_zeroes(&consts::ROWS)?;
        self.check_hidden_zeroes(&consts::COLS)?;
        self.check_hidden_zeroes(&consts::CELLS)
    }
}