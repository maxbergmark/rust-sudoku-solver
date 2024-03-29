#![warn(clippy::all)]

pub mod consts;
pub mod debug;
pub mod error;
pub mod hidden;
pub mod solver;
pub mod sudoku;
pub mod visible;

use error::SudokuError;
#[allow(unused)]
use rayon::prelude::*;
use std::{fmt::Write, fs::read_to_string, str::FromStr, time::Instant};
use sudoku::Sudoku;

fn read_lines(filename: &str) -> Result<Vec<Sudoku>, SudokuError> {
    read_to_string(filename)
        .map_err(|_| SudokuError::ParseError)?
        .lines()
        .skip(1)
        .map(Sudoku::from_str)
        .collect()
}

fn solve_all_in_file(filename: &str) -> Result<Vec<Sudoku>, SudokuError> {
    let sudokus = read_lines(filename)?;
    sudokus
        .into_iter()
        // .into_par_iter()
        .map(solver::solve)
        .collect()
}

fn benchmark_file(filename: &str) {
    let now = Instant::now();
    let solutions = solve_all_in_file(filename).unwrap();
    let elapsed = now.elapsed();
    let n = solutions.len();
    let time_per_puzzle = elapsed / u32::try_from(n).unwrap();

    let total_recursions: i32 = solutions.iter().map(|s| s.num_recursions).sum();
    let max_recursions: i32 = solutions.iter().map(|s| s.num_recursions).max().unwrap();
    let recursions_per_puzzle = f64::from(total_recursions) / n as f64;

    let total_guesses: i32 = solutions.iter().map(|s| s.guesses).sum();
    let avg_guesses = f64::from(total_guesses) / n as f64;

    print!("\r{filename:45}{n:>12}    {elapsed:10.2?}{time_per_puzzle:12.2?}");
    println!("{recursions_per_puzzle:15.4}{max_recursions:15}{avg_guesses:12.4}");
}

fn is_valid(s: &Sudoku, solution: &Sudoku) -> bool {
    s.digits
        .iter()
        .zip(solution.digits)
        .filter(|(&sudoku_digit, _)| sudoku_digit != 0)
        .all(|(&sudoku_digit, solution_digit)| sudoku_digit == solution_digit)
}

#[allow(unused)]
fn output_solutions(filename: &str) -> Result<String, SudokuError> {
    let sudokus = read_lines(filename)?;
    let mut ret = String::new();
    writeln!(ret, "{}", sudokus.len());
    sudokus
        .into_iter()
        .try_for_each(|sudoku| write_row(&mut ret, sudoku))
        .map_err(|_| SudokuError::ParseError)?;
    Ok(ret)
}

fn write_row(ret: &mut String, sudoku: Sudoku) -> std::fmt::Result {
    let solution = solver::solve(sudoku.clone()).unwrap_or(sudoku.clone());
    assert!(is_valid(&sudoku, &solution));
    writeln!(ret, "{sudoku},{solution}")
}

#[allow(unused)]
fn benchmark() {
    println!(
        "{:45}{:>12}    {:10}{:12}{:>15}{:>15}{:>12}",
        "filename",
        "num sudokus",
        "time",
        "per puzzle",
        "avg_recursions",
        "max_recursions",
        "avg_guesses"
    );
    benchmark_file("data-sets/easiest.txt");
    benchmark_file("data-sets/hard_sudokus.txt");
    benchmark_file("data-sets/all_17_clue_sudokus.txt");
    benchmark_file("data-sets/puzzles6_forum_hardest_1106.txt");
    benchmark_file("data-sets/ph1307.txt");
    // benchmark_file("data-sets/ph1910_01.txt");
    // benchmark_file("data-sets/ph1910_02.txt");
}

fn parse_and_expect(filename: &str, expected_digest: &str) {
    let output = output_solutions(filename).unwrap();
    let digest = md5::compute(output.as_bytes());
    assert_eq!(format!("{digest:x}"), expected_digest);
}

fn count_recursions(filename: &str, expected: i32) {
    let solutions = solve_all_in_file(filename).unwrap();
    let recursions = solutions.iter().map(|s| s.num_recursions).sum::<i32>();
    assert_eq!(expected, recursions);
}

fn validate_hashes() {
    parse_and_expect(
        "data-sets/hard_sudokus.txt",
        "3cb465ef6077c4fcab5bd6ae3bc50d62",
    );
    parse_and_expect("data-sets/hardest.txt", "6f5da3b2d03afe7d746514f9e6448c70");
    parse_and_expect(
        "data-sets/all_17_clue_sudokus.txt",
        "41704fd7d8fd0723a45ffbb2dbbfa488",
    );
}

fn validate_recursions() {
    count_recursions("data-sets/all_17_clue_sudokus.txt", 262_592);
    count_recursions("data-sets/easiest.txt", 0);
    count_recursions("data-sets/hard_sudokus.txt", 15_240);
    count_recursions("data-sets/puzzles6_forum_hardest_1106.txt", 83_611);
}

fn main() {
    benchmark();
    validate_hashes();
    validate_recursions();
    // output_solutions("data-sets/easiest.txt");
    // output_solutions("data-sets/all_17_clue_sudokus.txt");
    // output_solutions("data-sets/hardest.txt");
    // output_solutions("data-sets/test.txt");
    // output_solutions("data-sets/hard_sudokus.txt");
}
