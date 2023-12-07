use aocshared::aoc::aoc::*;
use itertools::Itertools;
use regex::Regex;

const YEAR: i32 = 2022;
const DAY: u32 = 05;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn parse_instructions(lines: &Vec<&str>) -> Vec<(i32, i32, i32)> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    lines
        .iter()
        .filter(|line| line.starts_with("move"))
        .map(|instr| {
            let caps = re.captures(instr).unwrap();
            let count = caps
                .get(1)
                .map(|s| s.as_str().parse::<i32>().unwrap())
                .unwrap();
            let from = caps
                .get(2)
                .map(|s| s.as_str().parse::<i32>().unwrap())
                .unwrap();
            let to = caps
                .get(3)
                .map(|s| s.as_str().parse::<i32>().unwrap())
                .unwrap();
            (count, from, to)
        })
        .collect_vec()
}

fn parse_state(lines: &Vec<&str>) -> Vec<Vec<char>> {
    let state = lines
        .iter()
        .take_while(|line| !line.trim().starts_with("1"))
        .map(|line| {
            line.replace("    ", "[&] ")
                .replace(" ", "")
                .replace("[", "")
                .replace("]", "")
        })
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    transpose(&state, '&')
}

fn part1(data: &String) -> String {
    let lines = get_lines_as_strs(data);
    let state = parse_state(&lines);
    let mut stacks = state
        .iter()
        .map(|stack| stack.iter().filter(|c| **c != '&').rev().collect_vec())
        .collect_vec();

    for (count, from, to) in parse_instructions(&lines) {
        for _ in 0..count {
            let cr = stacks[(from - 1) as usize].pop();
            match cr {
                Some(cr) => stacks[(to - 1) as usize].push(cr),
                None => (),
            }
        }
    }
    stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
}

fn part2(data: &String) -> String {
    let lines = get_lines_as_strs(data);
    let state = parse_state(&lines);
    let mut stacks = state
        .iter()
        .map(|stack| stack.iter().filter(|c| **c != '&').rev().collect_vec())
        .collect_vec();

    for (count, from, to) in parse_instructions(&lines) {
        (0..count)
            .into_iter()
            .map(|_| stacks[(from - 1) as usize].pop())
            .collect_vec()
            .iter()
            .rev()
            .for_each(|cr| match cr {
                Some(cr) => stacks[(to - 1) as usize].push(cr),
                None => (),
            })
    }
    stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2022_05_ep1() {
        assert_eq!("CMZ", part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_05_ep2() {
        assert_eq!("MCD", part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_05_rp1() {
        assert_eq!("VQZNJMWTR", part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_05_rp2() {
        assert_eq!("NLCDCLVMQ", part2(&get_input(YEAR, DAY)));
    }
}
