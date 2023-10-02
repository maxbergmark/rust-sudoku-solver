# Rust Sudoku Solver

A fast and efficient Sudoku solver written in Rust.

## Benchmark (single thread)
    filename                                      num sudokus    time      per puzzle
    data-sets/easiest.txt                               10000    25.01ms   2.50µs
    data-sets/hard_sudokus.txt                          10000    102.13ms  10.21µs
    data-sets/all_17_clue_sudokus.txt                   49151    937.23ms  19.07µs
    data-sets/puzzles6_forum_hardest_1106.txt             375    161.34ms  430.24µs
    data-sets/ph1307.txt                                18058    1.86s     103.13µs

## Benchmark (multi-threaded)

    filename                                      num sudokus    time      per puzzle
    data-sets/easiest.txt                               10000    9.85ms    985.00ns
    data-sets/hard_sudokus.txt                          10000    22.21ms   2.22µs
    data-sets/all_17_clue_sudokus.txt                   49151    165.41ms  3.37µs
    data-sets/puzzles6_forum_hardest_1106.txt             375    28.00ms   74.66µs
    data-sets/ph1307.txt                                18058    315.72ms  17.48µs
