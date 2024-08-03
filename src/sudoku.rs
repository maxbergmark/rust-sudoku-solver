use std::{fmt, str::FromStr};

use crate::{consts, error::Error};

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
                    .and_then(|d| char::from_digit(d, 10))
                    .unwrap_or('.')
            })
            .collect();
        write!(f, "{s}")
    }
}

impl FromStr for Sudoku {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sudoku = Self::default();

        for (i, c) in s.chars().enumerate() {
            if c != '.' && c != '0' {
                let digit = c.to_digit(10).ok_or(Error::ParseError)? as consts::BitWidth;
                sudoku.place(i, digit);
            }
        }
        Ok(sudoku)
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self {
            bitboard: [consts::MASK; consts::SIZE],
            digits: [0; consts::SIZE],
            num_digits: 0,
            num_recursions: 0,
            guesses: 0,
        }
    }
}

impl Sudoku {
    #[inline]
    #[must_use]
    pub const fn is_solved(&self) -> bool {
        self.num_digits == consts::SIZE
    }

    #[inline]
    pub fn place(&mut self, idx: usize, digit: consts::BitWidth) {
        self.digits[idx] = digit;
        self.bitboard[idx] = 0;
        self.num_digits += 1;
        let mask = consts::MASK ^ (1 << digit);

        for neighbor in consts::NEIGHBORS[idx] {
            self.bitboard[neighbor] &= mask;
        }
    }

    #[inline]
    pub(crate) fn store_stats(&mut self, error: Error) -> Error {
        if let Error::NoSolution {
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
#[allow(clippy::panic_in_result_fn)]
mod tests {
    use super::super::*;
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".................................................................................",
        "500000000000000000000000000000000000000000000000000000000000000000000000000000000",
        0,
        5,
        false
    )]
    #[case(
        "5................................................................................",
        "540000000000000000000000000000000000000000000000000000000000000000000000000000000",
        1,
        4,
        false
    )]
    #[case(
        "5................................................................................",
        "400000000000000000000000000000000000000000000000000000000000000000000000000000000",
        0,
        4,
        false
    )]
    #[case(
        "97856231413649782552431876974965318238572194661284957389723645146198523725317469.",
        "978562314136497825524318769749653182385721946612849573897236451461985237253174698",
        80,
        8,
        true
    )]
    fn test_place(
        #[case] input: &str,
        #[case] expected: &str,
        #[case] idx: usize,
        #[case] digit: consts::BitWidth,
        #[case] is_solved: bool,
    ) -> Result<()> {
        let mut sudoku = Sudoku::from_str(input)?;

        sudoku.place(idx, digit);
        assert_eq!(sudoku.to_string(), expected);
        assert_eq!(sudoku.is_solved(), is_solved);
        Ok(())
    }
}
