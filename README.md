bradegler's Advent of Code Repository

This is a repository containing solutions for the 2020, 2021, 2022, 2023, 2024 and 2025 Advent of Code
(https://adventofcode.com/).

## Generate a files for a new day

To generate a files for a new day, run the following command:

```shell
cargo run --bin daygen -- <day> <year>
```

## Running the code

To run a solution, run the following command:

```shell
cargo test --bin aoc<year>_<day> --release

cargo test --bin aoc2025_01 --release
```
