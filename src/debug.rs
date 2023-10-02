use crate::{sudoku::Sudoku, consts};

use std::fmt::Write;



impl Sudoku {

    pub fn pretty_print(&self) -> String {
        let mut ret = String::new();
        for i in 0..consts::WIDTH {
            if i % 3 == 0 {
                writeln!(ret, "+---+---+---+").unwrap();
            }
            for j in 0..consts::WIDTH {
                if j % 3 == 0 {
                    write!(ret, "|").unwrap();
                }
                if self.digits[consts::WIDTH * i + j] != 0 {
                    write!(ret, "{}", self.digits[9*i+j]).unwrap();
                } else {
                    write!(ret, " ").unwrap();
                }
            }
            writeln!(ret, "|").unwrap();
        }
        writeln!(ret, "+---+---+---+").unwrap();
        ret
    }

    pub fn pretty_print_alternatives(&self) -> String {
        let mut ret = String::new();
        let mut board = vec![vec![' '; 3 * consts::WIDTH]; 3 * consts::WIDTH];
        for i in 0..consts::WIDTH {
            for j in 0..consts::WIDTH {
                let digit = self.digits[consts::WIDTH * i + j];
                if self.digits[consts::WIDTH * i + j] != 0 {
                    for d in 1..=9 {
                        board[3*i + (d-1) / 3][3*j + (d-1) % 3] = 'x';
                    }
                    board[3*i+1][3*j+1] = char::from_digit(digit as u32, 10).unwrap();
                } else {
                    for d in 1..=9 {
                        if self.bitboard[9*i+j] & (1 << d) > 0 {
                            board[3*i + (d-1) / 3][3*j + (d-1) % 3] = char::from_digit(d as u32, 10).unwrap()
                        }
                    }
                }
            }
        }
        for i in 0..3 * consts::WIDTH {
            if i % 3 == 0 {
                writeln!(ret, "+---+---+---+---+---+---+---+---+---+").unwrap();
            }
            for j in 0..3 * consts::WIDTH {
                if j % 3 == 0 {
                    write!(ret, "|").unwrap();
                }
                write!(ret, "{}", board[i][j]).unwrap();
            }
            writeln!(ret, "|").unwrap();
        }
        writeln!(ret, "+---+---+---+---+---+---+---+---+---+").unwrap();
        ret
    }    
}