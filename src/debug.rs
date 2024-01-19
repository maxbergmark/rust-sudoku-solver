use crate::{sudoku::Sudoku, consts};

use std::fmt::Write;

pub fn pretty_print(sudoku: &Sudoku) -> Option<String> {
    let mut ret = String::new();
    for i in 0..consts::WIDTH {
        if i % 3 == 0 {
            writeln!(ret, "+---+---+---+").ok()?;
        }
        for j in 0..consts::WIDTH {
            if j % 3 == 0 {
                write!(ret, "|").ok()?;
            }
            if sudoku.digits[consts::WIDTH * i + j] != 0 {
                write!(ret, "{}", sudoku.digits[9*i+j]).ok()?;
            } else {
                write!(ret, " ").ok()?;
            }
        }
        writeln!(ret, "|").ok()?;
    }
    writeln!(ret, "+---+---+---+").ok()?;
    Some(ret)
}

pub fn pretty_print_alternatives(sudoku: &Sudoku) -> Option<String> {
    let mut ret = String::new();
    let mut board = vec![vec![' '; 3 * consts::WIDTH]; 3 * consts::WIDTH];
    for i in 0..consts::WIDTH {
        for j in 0..consts::WIDTH {
            let digit = sudoku.digits[consts::WIDTH * i + j];
            if sudoku.digits[consts::WIDTH * i + j] != 0 {
                for d in 1..=9 {
                    board[3*i + (d-1) / 3][3*j + (d-1) % 3] = 'x';
                }
                board[3*i+1][3*j+1] = char::from_digit(digit as u32, 10)?;
            } else {
                for d in 1..=9 {
                    if sudoku.bitboard[9*i+j] & (1 << d) > 0 {
                        board[3*i + (d-1) / 3][3*j + (d-1) % 3] = char::from_digit(d as u32, 10)?
                    }
                }
            }
        }
    }

    for (i, row) in board.iter().enumerate().take(3 * consts::WIDTH) {
        if i % 3 == 0 {
            writeln!(ret, "+---+---+---+---+---+---+---+---+---+").ok()?;
        }
        for (j, cell) in row.iter().enumerate().take(3 * consts::WIDTH) {
            if j % 3 == 0 {
                write!(ret, "|").ok()?;
            }
            write!(ret, "{}", cell).ok()?;
        }
        writeln!(ret, "|").ok()?;
    }
    writeln!(ret, "+---+---+---+---+---+---+---+---+---+").ok()?;
    Some(ret)
}    
