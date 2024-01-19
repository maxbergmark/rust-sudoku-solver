use std::{fmt, str::FromStr};

use crate::{consts, error::SudokuError};

#[derive(Debug, Clone)]
pub struct Sudoku {
    pub bitboard: [consts::BitWidth; consts::SIZE],
    pub digits: [consts::BitWidth; consts::SIZE],
    num_digits: usize,
    pub num_recursions: i32,
    pub guesses: i32,
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self
            .digits
            .iter()
            .map(|&digit| {
                u32::try_from(digit)
                    .ok()
                    .and_then(|digit| char::from_digit(digit, 10))
                    .unwrap_or('?')
            })
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
    pub(crate) fn is_solved(&self) -> bool {
        self.num_digits == consts::SIZE
    }

    #[inline]
    pub(crate) fn place(&mut self, idx: usize, digit: consts::BitWidth) {
        self.digits[idx] = digit;
        self.bitboard[idx] = 0;
        self.num_digits += 1;
        let mask = consts::MASK ^ (1 << digit);

        for neighbor in consts::NEIGHBORS[idx] {
            self.bitboard[neighbor] &= mask;
        }
    }

    #[inline]
    pub(crate) fn store_stats(&mut self, error: SudokuError) -> SudokuError {
        if let SudokuError::NoSolution {
            num_recursions,
            guesses,
        } = error
        {
            self.num_recursions = num_recursions;
            self.guesses = guesses;
        }
        error
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".................................................................................",
        "500000000000000000000000000000000000000000000000000000000000000000000000000000000",
        0,
        5
    )]
    #[case(
        "5................................................................................",
        "400000000000000000000000000000000000000000000000000000000000000000000000000000000",
        0,
        4
    )]
    fn test_place(
        #[case] input: &str,
        #[case] expected: &str,
        #[case] idx: usize,
        #[case] digit: consts::BitWidth,
    ) {
        let mut sudoku = Sudoku::from_str(input).unwrap();

        let _ = sudoku.place(idx, digit);
        assert_eq!(sudoku.to_string(), expected);
    }
}
