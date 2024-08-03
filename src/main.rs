#![warn(
    // missing_docs,
    // unreachable_pub,
    keyword_idents,
    unexpected_cfgs,
    missing_copy_implementations,
    missing_debug_implementations,
    non_ascii_idents,
    noop_method_call,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    future_incompatible,
    nonstandard_style,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
)]

use derive_more as _;
use rstest as _;

#[allow(unused)]
use rayon::prelude::*;
use rust_sudoku_solver::{solver, Error, Result, Sudoku};
use std::{fmt::Write, fs::read_to_string, str::FromStr, time::Instant};

fn read_lines(filename: &str) -> Result<Vec<Sudoku>> {
    read_to_string(filename)
        .map_err(Error::Io)?
        .lines()
        .skip(1)
        .map(Sudoku::from_str)
        .collect()
}

fn solve_all_in_file(filename: &str) -> Result<(Vec<Sudoku>, Vec<Sudoku>)> {
    let sudokus = read_lines(filename)?;
    let solutions: Result<Vec<Sudoku>> = sudokus
        .iter()
        .cloned()
        // .into_par_iter()
        .map(solver::solve)
        .collect();
    Ok((sudokus, solutions?))
}

#[allow(unused)]
#[allow(clippy::print_stdout)]
fn print_hardest_sudoku(sudokus: &[Sudoku], solutions: &[Sudoku]) -> Result<()> {
    let (hardest_sudoku, _) = sudokus
        .iter()
        .zip(solutions.iter())
        .max_by_key(|(_, solution)| solution.num_recursions)
        .ok_or(Error::SolveError)?;

    println!("hardest sudoku: {hardest_sudoku}");
    Ok(())
}

#[allow(clippy::print_stdout)]
#[allow(clippy::use_debug)]
fn benchmark_file(filename: &str) -> Result<()> {
    let now = Instant::now();
    #[allow(unused)]
    let (sudokus, solutions) = solve_all_in_file(filename)?;
    let elapsed = now.elapsed();
    let n = solutions.len();
    let time_per_puzzle = elapsed / u32::try_from(n)?;

    // print_hardest_sudoku(&sudokus, &solutions)?;

    let total_recursions: i32 = solutions.iter().map(|s| s.num_recursions).sum();
    let max_recursions: i32 = solutions
        .iter()
        .map(|s| s.num_recursions)
        .max()
        .ok_or(Error::SolveError)?;
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
fn output_solutions(filename: &str) -> Result<String> {
    let sudokus = read_lines(filename)?;
    let mut ret = String::new();
    writeln!(ret, "{}", sudokus.len());
    sudokus
        .into_iter()
        .try_for_each(|sudoku| write_row(&mut ret, &sudoku))?;
    Ok(ret)
}

fn write_row(ret: &mut String, sudoku: &Sudoku) -> Result<()> {
    let solution = solver::solve(sudoku.clone())?;
    if !is_valid(sudoku, &solution) {
        return Err(Error::SolveError);
    }
    Ok(writeln!(ret, "{sudoku},{solution}")?)
}

#[allow(clippy::print_stdout)]
#[allow(clippy::use_debug)]
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

#[allow(clippy::print_stdout)]
fn parse_and_expect(filename: &str, expected_digest: &str) -> Result<()> {
    let output = output_solutions(filename).unwrap_or_default();
    let digest = md5::compute(output.as_bytes());
    if format!("{digest:x}") != expected_digest {
        println!("expected digest: {expected_digest}");
        println!("actual digest:   {digest:x}");
        println!("output: {output}");
        return Err(Error::SolveError);
    }
    Ok(())
}

struct SolutionData {
    filename: String,
    expected_recursions: i32,
    recursions: i32,
    expected_guesses: i32,
    guesses: i32,
}

impl SolutionData {
    const fn is_valid(&self) -> bool {
        self.recursions <= self.expected_recursions && self.guesses <= self.expected_guesses
    }
}

#[allow(clippy::print_stdout)]
fn print_file_info(solution_data: &SolutionData) {
    println!("filename: {}", solution_data.filename);
    println!(
        "expected recursions: {:7}",
        solution_data.expected_recursions
    );
    println!("actual recursions:   {:7}", solution_data.recursions);
    println!("expected guesses:    {:7}", solution_data.expected_guesses);
    println!("actual guesses:      {:7}", solution_data.guesses);
}

#[allow(clippy::print_stdout)]
fn check_solution_data(solution_data: &SolutionData) -> Result<()> {
    if solution_data.is_valid() {
        println!(
            "recursions: {:7} <= {:7}",
            solution_data.recursions, solution_data.expected_recursions
        );
        println!(
            "guesses:    {:7} <= {:7}",
            solution_data.guesses, solution_data.expected_guesses
        );
        Ok(())
    } else {
        print_file_info(solution_data);
        Err(Error::SolveError)
    }
}

fn count_recursions(filename: &str, expected_recursions: i32, expected_guesses: i32) -> Result<()> {
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
    check_solution_data(&solution_data)
}

fn validate_hashes() -> Result<()> {
    parse_and_expect(
        "data-sets/hard_sudokus.txt",
        "3cb465ef6077c4fcab5bd6ae3bc50d62",
    )?;
    parse_and_expect("data-sets/hardest.txt", "6f5da3b2d03afe7d746514f9e6448c70")?;
    parse_and_expect(
        "data-sets/all_17_clue_sudokus.txt",
        "41704fd7d8fd0723a45ffbb2dbbfa488",
    )
}

fn validate_recursions() -> Result<()> {
    count_recursions("data-sets/all_17_clue_sudokus.txt", 262_592, 137_394)?;
    count_recursions("data-sets/easiest.txt", 0, 0)?;
    count_recursions("data-sets/hard_sudokus.txt", 15_240, 8730)?;
    count_recursions("data-sets/puzzles6_forum_hardest_1106.txt", 83_611, 53_226)
}

fn main() -> Result<()> {
    benchmark();
    validate_hashes()?;
    validate_recursions()?;
    // output_solutions("data-sets/easiest.txt");
    // output_solutions("data-sets/all_17_clue_sudokus.txt");
    // output_solutions("data-sets/hardest.txt");
    // output_solutions("data-sets/test.txt");
    // output_solutions("data-sets/hard_sudokus.txt");
    Ok(())
}
