#![warn(clippy::all)]

pub mod consts;
pub mod debug;
pub mod error;
pub mod hidden;
pub mod solver;
pub mod sudoku;
pub mod visible;
pub mod triples;

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

fn solve_all_in_file(filename: &str) -> Result<(Vec<Sudoku>, Vec<Sudoku>), SudokuError> {
    let sudokus = read_lines(filename)?;
    let solutions: Result<Vec<Sudoku>, SudokuError> = sudokus
        .iter()
        .cloned()
        // .into_par_iter()
        .map(solver::solve)
        .collect();
    Ok((sudokus, solutions?))
}

#[allow(unused)]
fn print_hardest_sudoku(sudokus: &[Sudoku], solutions: &[Sudoku]) -> Result<(), SudokuError> {
    let (hardest_sudoku, _) = sudokus.iter().zip(solutions.iter())
        .max_by_key(|(_, solution)| solution.num_recursions)
        .ok_or(SudokuError::SolveError)?;

    println!("hardest sudoku: {}", hardest_sudoku);
    Ok(())
}

fn benchmark_file(filename: &str) -> Result<(), SudokuError> {
    let now = Instant::now();
    #[allow(unused)]
    let (sudokus, solutions) = solve_all_in_file(filename)?;
    let elapsed = now.elapsed();
    let n = solutions.len();
    let time_per_puzzle = elapsed / u32::try_from(n).map_err(|_| SudokuError::SolveError)?;

    // print_hardest_sudoku(&sudokus, &solutions)?;

    let total_recursions: i32 = solutions.iter().map(|s| s.num_recursions).sum();
    let max_recursions: i32 = solutions
        .iter()
        .map(|s| s.num_recursions)
        .max()
        .ok_or(SudokuError::SolveError)?;
    let recursions_per_puzzle = f64::from(total_recursions) / n as f64;

    let total_guesses: i32 = solutions.iter().map(|s| s.guesses).sum();
    let avg_guesses = f64::from(total_guesses) / n as f64;

    print!("\r{filename:45}{n:>12}    {elapsed:10.2?}{time_per_puzzle:12.2?}");
    println!("{recursions_per_puzzle:15.4}{max_recursions:15}{avg_guesses:12.4}");
    Ok(())
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
    let output = output_solutions(filename).unwrap_or_default();
    let digest = md5::compute(output.as_bytes());
    if format!("{digest:x}") != expected_digest {
        println!("expected digest: {expected_digest}");
        println!("actual digest:   {digest:x}");
        println!("output: {}", output);
        panic!();
    }
    assert_eq!(format!("{digest:x}"), expected_digest);
}

struct SolutionData {
    filename: String,
    expected_recursions: i32,
    recursions: i32,
    expected_guesses: i32,
    guesses: i32,
}

impl SolutionData {
    fn is_valid(&self) -> bool {
        self.recursions <= self.expected_recursions && self.guesses <= self.expected_guesses
    }
}

fn print_file_info(solution_data: SolutionData) {
    println!("filename: {}", solution_data.filename);
    println!("expected recursions: {:7}", solution_data.expected_recursions);
    println!("actual recursions:   {:7}", solution_data.recursions);
    println!("expected guesses:    {:7}", solution_data.expected_guesses);
    println!("actual guesses:      {:7}", solution_data.guesses);
}

fn check_solution_data(solution_data: SolutionData) {
    if !solution_data.is_valid() {
        print_file_info(solution_data);
        panic!();
    } else {
        println!("recursions: {:7} <= {:7}", solution_data.recursions, solution_data.expected_recursions);
        println!("guesses:    {:7} <= {:7}", solution_data.guesses, solution_data.expected_guesses);
    }
}

fn count_recursions(filename: &str, expected_recursions: i32, expected_guesses: i32) {
    let (_, solutions) = solve_all_in_file(filename).unwrap_or_default();
    let recursions = solutions.iter().map(|s| s.num_recursions).sum::<i32>();
    let guesses = solutions.iter().map(|s| s.guesses).sum::<i32>();
    
    let solution_data = SolutionData {
        filename: filename.to_string(),
        expected_recursions,
        recursions,
        expected_guesses,
        guesses,
    };
    check_solution_data(solution_data);
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
    count_recursions("data-sets/all_17_clue_sudokus.txt", 262_592, 137_394);
    count_recursions("data-sets/easiest.txt", 0, 0);
    count_recursions("data-sets/hard_sudokus.txt", 15_240, 8730);
    count_recursions("data-sets/puzzles6_forum_hardest_1106.txt", 83_611, 53_226);
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
