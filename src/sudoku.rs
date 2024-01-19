use std::{fmt, str::FromStr};

use crate::{consts, hidden::{place_all_hidden_zeroes, place_all_hidden_singles}, visible::{place_all_visible_doubles, place_all_visible_singles}};


#[derive(Debug, Clone)]
pub struct Sudoku {
    pub bitboard: [consts::BitWidth; consts::SIZE],
    pub digits: [consts::BitWidth; consts::SIZE],
    num_digits: i32,
    pub num_recursions: i32,
    pub guesses: i32,
}

#[derive(Debug)]
pub enum SudokuError {
    ParseError,
    IndexError,
    NoSolution {
        num_recursions: i32,
        guesses: i32,
    }
}

impl From<&Sudoku> for SudokuError {
    fn from(sudoku: &Sudoku) -> Self {
        SudokuError::NoSolution {
            num_recursions: sudoku.num_recursions,
            guesses: sudoku.guesses, 
        }
    }
}

impl From<&mut Sudoku> for SudokuError {
    fn from(sudoku: &mut Sudoku) -> Self {
        SudokuError::NoSolution {
            num_recursions: sudoku.num_recursions,
            guesses: sudoku.guesses, 
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.digits.iter()
            .map(|&d| char::from_digit(d as u32, 10).unwrap_or('?'))
            .collect();
        write!(f, "{s}")
    }
}

impl FromStr for Sudoku {
    type Err = SudokuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sudoku = Sudoku { 
            bitboard: [consts::MASK; consts::SIZE],
            digits: [0; consts::SIZE], 
            num_digits: 0,
            num_recursions: 0,
            guesses: 0,            
        };
        for (i, c) in s.chars().enumerate() {
            let digit = if c == '.' {
                0
            } else {
                c.to_digit(10).ok_or(SudokuError::ParseError)? as consts::BitWidth
            };

            if digit != 0 {
                sudoku.place(i, digit);
            }
        }
        Ok(sudoku)
    }
}

impl Sudoku {

    #[inline]
    fn is_solved(&self) -> bool {
        self.num_digits == consts::SIZE as i32
    }

    #[inline]
    pub(crate) fn place(&mut self, i: usize, n: consts::BitWidth) {
        if let Some(digit) = self.digits.get_mut(i) {
            *digit = n
        }

        self.num_digits += 1;
        let mask = consts::MASK ^ (1 << n);
        for neighbor in consts::NEIGHBORS[i] {
            self.bitboard[neighbor] &= mask;
        }
        self.bitboard[i] = 0;
    }

    #[inline]
    fn unit_propagate(&mut self, i: usize) -> Result<(), SudokuError> {
        for neighbor in consts::NEIGHBORS[i] {
            let digit = self.digits[neighbor];
            let bitboard = self.bitboard[neighbor];
            if digit == 0 && bitboard == 0 {
                return Err(SudokuError::from(self));
            } else if digit == 0 && bitboard.count_ones() == 1 {
                let digit = self.bitboard[neighbor].trailing_zeros() as consts::BitWidth;
                self.place_and_propagate(neighbor, digit)?;
            }
        }
        Ok(())
    }

    pub(crate) fn place_and_propagate(&mut self, i: usize, n: consts::BitWidth) -> Result<(), SudokuError> {
        self.place(i, n);
        self.unit_propagate(i)
    }

    #[inline]
    fn store_stats(&mut self, error: SudokuError) -> SudokuError {
        if let SudokuError::NoSolution { num_recursions, guesses } = error {
            self.num_recursions = num_recursions;
            self.guesses = guesses;
        }
        error
    }

    #[inline]
    fn check_branch(&mut self, idx: usize, n: u16) -> Result<Sudoku, SudokuError> {
        let mut cloned_board = self.clone();
        cloned_board.place_and_propagate(idx, n)?;
        cloned_board.solve_recursive()
            .map_err(|error| self.store_stats(error))
    }

    fn branch_possibilities(&mut self, idx: usize) -> Result<Sudoku, SudokuError> {
        let bitboard = self.bitboard[idx];
        self.guesses += (bitboard.count_ones() > 1) as i32;
        let start = bitboard.trailing_zeros() as u16;
        let end = 16 - bitboard.leading_zeros() as u16;

        (start..end)
            .filter(|&n| bitboard & (1<<n) > 0)
            .map(|n| self.check_branch(idx, n))
            .find_map(Result::ok)
            .ok_or(SudokuError::from(self))
    }

    fn solve_recursive(&mut self) -> Result<Sudoku, SudokuError> {
        self.num_recursions += 1;
        check_constraints_new(self)?;
        if self.is_solved() {
            Ok(self.clone())
        } else if let Some(idx) = get_next_idx(self) {
            self.branch_possibilities(idx)
        } else {
            Err(SudokuError::from(self))
        }
    }
}

pub fn solve(mut sudoku: Sudoku) -> Result<Sudoku, SudokuError> {
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
    sudoku.solve_recursive()
}

fn get_next_idx(sudoku: &Sudoku) -> Option<usize> {
    (0..consts::SIZE)
        .filter(|&i| sudoku.digits[i] == 0)
        .map(|i| (i, sudoku.bitboard[i].count_ones()))
        .min_by_key(|&(_, num_possibilities)| num_possibilities)
        .map(|(idx, _)| idx)
}


#[inline]
fn check_constraints_new(sudoku: &mut Sudoku) -> Result<(), SudokuError> {
    place_all_hidden_zeroes(sudoku)?;
    place_all_hidden_singles(sudoku)?;
    place_all_visible_singles(sudoku)?;
    place_all_visible_doubles(sudoku)
}
