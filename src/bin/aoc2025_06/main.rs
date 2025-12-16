use core::num;

use aocshared::{aoc::aoc::*, grid::grid::Grid};
use itertools::Itertools;
use regex::Regex;

const YEAR: i32 = 2025;
const DAY: u32 = 06;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let re = Regex::new(r" +").unwrap();
    let raw = re.replace_all(data, ",");
    let lines = get_lines_as_strs_rm_empty(&raw);
    let grid = lines[..lines.len() - 1]
        .into_iter()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<u64>())
                .filter(|u| u.is_ok())
                .map(|y| y.unwrap())
                .collect_vec()
        })
        .collect_vec();
    let operators = lines[lines.len() - 1].split(",").collect_vec();
    let mut results = vec![];
    for col in 0..operators.len() {
        if operators[col] == "+" {
            let mut result = 0;
            for row in 0..grid.len() {
                result += grid[row][col];
            }
            results.push(result);
        }
        if operators[col] == "*" {
            let mut result = 1;
            for row in 0..grid.len() {
                result *= grid[row][col];
            }
            results.push(result);
        }
    }

    results.into_iter().sum()
}

fn part2(data: &String) -> u64 {
    let grid = Grid::new(get_lines_as_grid_char(data));
    let mut results: Vec<u64> = vec![];
    let mut calc = vec![];
    for col in (0..grid.width()).rev() {
        let mut numbers = vec![];
        for row in 0..grid.height() - 1 {
            numbers.push(grid.at((row, col)))
        }
        let value: String = numbers.into_iter().filter(|c| *c != ' ').collect();
        if value.len() == 0 {
            continue;
        }
        let num = value.parse::<u64>().unwrap();
        let op = grid.at((grid.height() - 1, col));
        calc.push(num);
        if op == '+' {
            let v = calc.iter().fold(0, |acc, n| acc + n);
            results.push(v);
            calc.clear();
        }
        if op == '*' {
            let v = calc.iter().fold(1, |acc, n| acc * n);
            results.push(v);
            calc.clear();
        }
    }
    results.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_06_ep1() {
        assert_eq!(4277556, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_06_ep2() {
        assert_eq!(3263827, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_06_rp1() {
        assert_eq!(5227286044585, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_06_rp2() {
        assert_eq!(10227753257799, part2(&get_input(YEAR, DAY)));
    }
}
