use aocshared::aoc::aoc::*;
use itertools::{repeat_n, Itertools};

const YEAR: i32 = 2024;
const DAY: u32 = 07;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Operators {
    Add,
    Mul,
    Concat,
}

fn part1(data: &String) -> u64 {
    let contents = get_lines_as_strs_rm_empty(data);
    contents
        .iter()
        .map(parse_line)
        .map(|(amount, values)| calculate(vec![Operators::Add, Operators::Mul], amount, values))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .sum::<u64>()
}

fn parse_line(data: &&str) -> (u64, Vec<u64>) {
    let parts = data.split_once(":").unwrap();
    let amount = parts.0.parse::<u64>().unwrap();
    let values = parts
        .1
        .trim_start()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    (amount, values)
}

fn part2(data: &String) -> u64 {
    let contents = get_lines_as_strs_rm_empty(data);
    contents
        .iter()
        .map(parse_line)
        .map(|(amount, values)| {
            calculate(
                vec![Operators::Add, Operators::Mul, Operators::Concat],
                amount,
                values,
            )
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .sum::<u64>()
}

fn calculate(ops: Vec<Operators>, amount: u64, values: Vec<u64>) -> Option<u64> {
    let parts = values.len();
    repeat_n(ops.iter(), parts - 1)
        .multi_cartesian_product()
        .map(|ops| {
            let mut calcd_val = values[0];
            for idx in 1..parts {
                let op = ops[idx - 1];
                match op {
                    Operators::Add => calcd_val += values[idx],
                    Operators::Mul => calcd_val *= values[idx],
                    Operators::Concat => {
                        calcd_val = format!("{}{}", calcd_val, values[idx])
                            .parse::<u64>()
                            .unwrap()
                    }
                }
            }
            calcd_val
        })
        .any(|val| val == amount)
        .then_some(amount)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2024_07_ep1() {
        assert_eq!(3749, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_07_ep2() {
        assert_eq!(11387, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_07_rp1() {
        assert_eq!(4555081946288, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_07_rp2() {
        assert_eq!(227921760109726, part2(&get_input(YEAR, DAY)));
    }
}
