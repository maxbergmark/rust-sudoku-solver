# Rust Sudoku Solver

A fast and efficient Sudoku solver written in Rust.

## Benchmark (single thread)
    filename                                      num sudokus    time      per puzzle
    data-sets/easiest.txt                               10000    16.14ms   1.61µs
    data-sets/hard_sudokus.txt                          10000    73.80ms   7.38µs
    data-sets/all_17_clue_sudokus.txt                   49151    563.94ms  11.47µs
    data-sets/puzzles6_forum_hardest_1106.txt             375    133.84ms  356.90µs
    data-sets/ph1307.txt                                18058    1.55s     86.09µs

## Benchmark (multi-threaded)

    filename                                      num sudokus    time      per puzzle
    data-sets/easiest.txt                               10000    10.38ms   1.04µs
    data-sets/hard_sudokus.txt                          10000    21.85ms   2.19µs
    data-sets/all_17_clue_sudokus.txt                   49151    112.02ms  2.28µs
    data-sets/puzzles6_forum_hardest_1106.txt             375    21.99ms   58.63µs
    data-sets/ph1307.txt                                18058    260.48ms  14.42µs
