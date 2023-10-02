use std::fmt;

use crate::consts;


#[derive(Debug, Clone, Copy)]
pub struct Sudoku {
    pub bitboard: [consts::BitWidth; consts::SIZE],
    pub digits: [consts::BitWidth; consts::SIZE],
    num_digits: i32,
    pub num_recursions: i32,
    pub guesses: i32,
}

#[derive(Debug)]
pub struct SudokuError {
    pub digit: u16,
    pub index: usize,
    pub num_recursions: i32,
    pub guesses: i32,
}

impl fmt::Display for Sudoku {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.digits.iter()
            .map(|d| char::from_digit(*d as u32, 10).unwrap())
            .collect();
        write!(f, "{s}")
    }
}

impl fmt::Display for SudokuError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // placeholder error message
        write!(f, "incorrect_sudoku")
    }
}


impl Sudoku {

    pub fn new(input: &str) -> Self {
        let mut sudoku = Sudoku { 
            bitboard: [consts::MASK; consts::SIZE],
            digits: [0; consts::SIZE], 
            num_digits: 0,
            num_recursions: 0,
            guesses: 0,            
        };
        for (i, c) in input.chars().enumerate() {
            if let '1'..='9' = c {
                let digit = c.to_digit(10).unwrap() as consts::BitWidth;
                sudoku.place(i, digit);
            }
        }
        sudoku
    }

    #[inline]
    fn is_solved(&self) -> bool {
        self.num_digits == i32::try_from(consts::SIZE).unwrap()
    }

    pub(crate) fn place(&mut self, i: usize, n: consts::BitWidth) {
        self.digits[i] = n;
        self.num_digits += 1;
        let mask = consts::MASK ^ (1 << n);
        for neighbor in consts::NEIGHBORS[i] {
            self.bitboard[neighbor] &= mask;
        }
        self.bitboard[i] = 0;
    }

    #[inline]
    fn unit_propagate(mut self, i: usize) -> Result<Self, SudokuError> {
        for neighbor in consts::NEIGHBORS[i] {
            let bitboard = self.bitboard[neighbor];
            if (self.digits[neighbor] == 0) & (bitboard == 0) {
                return Err(SudokuError { 
                    digit: 0,
                    index: i,
                    num_recursions: self.num_recursions, 
                    guesses: self.guesses 
                })
            } else if (self.digits[neighbor] == 0) & (bitboard.count_ones() == 1) {
                let digit = self.bitboard[neighbor].trailing_zeros() as consts::BitWidth;
                self = self.place_and_copy(neighbor, digit)?;        
            }
        }
        Ok(self)
    }

    pub(crate) fn place_and_copy(mut self, i: usize, n: consts::BitWidth) -> Result<Self, SudokuError> {
        self.digits[i] = n;
        self.num_digits += 1;
        let mask = consts::MASK ^ (1 << n);
        self.bitboard[i] = 0;

        for neighbor in consts::NEIGHBORS[i] {
            self.bitboard[neighbor] &= mask;
        }

        self.unit_propagate(i)
    }

    fn get_next_idx(&self) -> Option<usize> {
        let mut min_idx = None;
        let mut min_options = 9;
        for idx in 0..consts::SIZE {
            if self.digits[idx] != 0 {
                continue;
            }
            let options = self.bitboard[idx].count_ones();
            if options < min_options {
                min_options = options;
                min_idx = Some(idx);
                if options == 1 {
                    return min_idx;
                }
            }
        }
        min_idx
    }

    fn check_constraints(&mut self) -> Result<(), SudokuError> {
        self.check_all_hidden_zeroes()?;
        self.check_all_hidden_singles()?;
        self.check_all_visible_singles()?;
        self.check_all_visible_doubles()

    }

    fn solve_recursive(mut self) -> Result<Self, SudokuError> {
        self.num_recursions += 1;
        self.check_constraints()?;
        if self.is_solved() {
            return Ok(self);
        }

        if let Some(idx) = self.get_next_idx() {
            let bitboard = self.bitboard[idx];
            self.guesses += (bitboard.count_ones() > 1) as i32;
            let start = bitboard.trailing_zeros() as u16;
            let end = 16 - bitboard.leading_zeros() as u16;
            // let start = 0;
            // let end = 10;
            for n in start..end {
                if self.bitboard[idx] & (1<<n) == 0 {
                    continue;
                }
                let attempt_placement = self.place_and_copy(idx, n);
                if let Ok(new_board) = attempt_placement {
                    match new_board.solve_recursive() {
                        Ok(solution) => return Ok(solution),
                        Err(err) => {
                            self.num_recursions = err.num_recursions;
                            self.guesses = err.guesses;
                        },
                    }    
                }
            }
        }

        Err(SudokuError { 
            digit: 0,
            index: 0,
            num_recursions: self.num_recursions, 
            guesses: self.guesses 
        })
    }

    pub fn solve(mut self) -> Result<Self, SudokuError> {
        // self.check_all_visible_singles()?;
        // if self.is_solved() {
            // return Ok(self);
        // }
        self.check_all_hidden_singles()?;
        if self.is_solved() {
            return Ok(self);
        }
        self.check_all_visible_doubles()?;
        // self.check_all_visible_n(2)?;
        // if self.num_digits < 50 {
            // self.check_all_visible_n(3)?;
            // self.check_all_visible_n(4)?;
            // self.check_all_visible_n(5)?;
            // self.check_all_visible_n(6)?;
            // self.check_all_visible_n(7)?;
            // self.check_all_visible_n(8)?;
        // }
        self.check_all_hidden_singles()?;
        // self.check_all_visible_singles()?;
        if self.is_solved() {
            return Ok(self);
        }
        self.solve_recursive()
    }

}