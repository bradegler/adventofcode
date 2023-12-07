use aocshared::aoc::aoc::*;
use std::collections::HashSet;

const YEAR: i32 = 2020;
const DAY: u32 = 06;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> usize {
    let mut input = data.clone();
    input.push_str("\n");
    input
        .replace("\n\n", "|")
        .replace("\n", "")
        .split("|")
        .map(|x| x.chars().collect::<HashSet<char>>().len())
        .sum::<usize>()
}

fn part2(data: &String) -> usize {
    let mut input = data.clone();
    input.push_str("\n");
    input
        .replace("\n\n", "|")
        .replace("\n", " ")
        .split("|")
        .map(|x| {
            x.replace("|", "")
                .split(" ")
                .map(|y| y.chars().collect::<HashSet<char>>())
                .reduce(|acc, x| acc.intersection(&x).cloned().collect::<HashSet<char>>())
                .map(|x| x.len())
                .unwrap_or(0)
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2020_06_ep1() {
        assert_eq!(11, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_06_ep2() {
        assert_eq!(6, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_06_rp1() {
        assert_eq!(6630, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_06_rp2() {
        assert_eq!(3437, part2(&get_input(YEAR, DAY)));
    }
}
