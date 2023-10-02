pub mod consts;
pub mod sudoku;
pub mod hidden;
pub mod visible;
pub mod debug;

#[allow(unused)]
use rayon::prelude::*;
use std::{fs::read_to_string, fmt::Write, time::Instant};
use sudoku::Sudoku;


fn read_lines(filename: &str) -> Vec<Sudoku> {
    read_to_string(filename).unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            Sudoku::new(line)
        }).collect()
}

fn solve_all_in_file(filename: &str) -> Vec<Sudoku> {
    let sudokus = read_lines(filename);
    sudokus
        .into_iter()
        // .into_par_iter()
        .enumerate()
        .map(|(_i, s)| {
            s.solve().unwrap()
        }).collect()
}

fn benchmark_file(filename: &str) {
    let now = Instant::now();
    let solutions = solve_all_in_file(filename);
    let elapsed = now.elapsed();
    let n = solutions.len();
    let time_per_puzzle = elapsed / u32::try_from(n).unwrap();

    let total_recursions: i32 = solutions.iter().map(|&s| s.num_recursions).sum();
    let max_recursions: i32 = solutions.iter().map(|&s| s.num_recursions).max().unwrap();
    let recursions_per_puzzle = total_recursions as f32 / n as f32;

    let total_guesses: i32 = solutions.iter().map(|&s| s.guesses).sum();
    let avg_guesses = total_guesses as f32 / n as f32;

    println!("\r{filename:45}{n:>12}    {elapsed:10.2?}{time_per_puzzle:12.2?}{recursions_per_puzzle:15.4}{max_recursions:15}{avg_guesses:12.4}");
}
fn validate(s: &Sudoku, solution: &Sudoku) -> bool {
    for (&d1, d2) in s.digits.iter().zip(solution.digits) {
        if d1 != 0 && d1 != d2 {
            return false;
        }
    }
    true
}

#[allow(unused)]
fn output_solutions(filename: &str) -> String {
    let sudokus = read_lines(filename);
    let mut ret = String::new();
    writeln!(ret, "{}", sudokus.len());
    sudokus.iter().enumerate().for_each(|(i, s)| {
        let solution = s.solve().unwrap();
        assert!(validate(s, &solution));
        writeln!(ret, "{s},{solution}");
    });
    print!("{ret}");
    ret
}

#[allow(unused)]
fn benchmark() {
    println!("{:45}{:>12}    {:10}{:12}{:>15}{:>15}{:>12}", "filename", "num sudokus", "time", "per puzzle", "avg_recursions", "max_recursions", "avg_guesses");
    benchmark_file("data-sets/easiest.txt");
    benchmark_file("data-sets/hard_sudokus.txt");
    benchmark_file("data-sets/all_17_clue_sudokus.txt");
    // benchmark_file("data-sets/hardest.txt");
    benchmark_file("data-sets/puzzles6_forum_hardest_1106.txt");
    benchmark_file("data-sets/ph1307.txt");
    // benchmark_file("data-sets/ph1910_01.txt");
    // solve_all_in_file("data-sets/ph1910_02.txt");
}

fn main() {
    benchmark();
    // output_solutions("data-sets/easiest.txt");
    // output_solutions("data-sets/all_17_clue_sudokus.txt");
    // output_solutions("data-sets/hardest.txt");
    // output_solutions("data-sets/test.txt");
    // output_solutions("data-sets/hard_sudokus.txt");
}

#[cfg(test)]
mod tests {
    use super::*;
    use md5;

    fn parse_and_expect(filename: &str, expected_digest: &str) {
        let output = output_solutions(filename);
        let digest = md5::compute(output.as_bytes());
        assert_eq!(format!("{:x}", digest), expected_digest);

    }

    #[test]
    fn test_hard() {
        parse_and_expect(
            "data-sets/hard_sudokus.txt", 
            "3cb465ef6077c4fcab5bd6ae3bc50d62"
        );
    }

    #[test]
    fn test_hardest() {
        parse_and_expect(
            "data-sets/hardest.txt", 
            "6f5da3b2d03afe7d746514f9e6448c70"
        );
    }

    #[test]
    fn test_17() {
        parse_and_expect(
            "data-sets/all_17_clue_sudokus.txt", 
            "41704fd7d8fd0723a45ffbb2dbbfa488"
        );
    }
}
