use aocshared::aoc::aoc::*;
use std::collections::HashMap;

const YEAR: i32 = 2021;
const DAY: u32 = 10;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    let mut scores = HashMap::new();
    scores.insert(')', 3);
    scores.insert(']', 57);
    scores.insert('}', 1197);
    scores.insert('>', 25137);

    let openers = vec!['(', '[', '{', '<'];
    let closers = vec![')', ']', '}', '>'];

    let input = get_lines_as_vec_chars(data);

    let mut result = 0;
    for line in input {
        let mut stack: Vec<char> = Vec::new();
        for c in line {
            if openers.contains(&c) {
                stack.push(c);
            } else {
                let last_open = stack.pop().unwrap();
                let last_open_index = openers.iter().position(|x| *x == last_open).unwrap();
                let closer_index = closers.iter().position(|x| *x == c).unwrap();
                if last_open_index != closer_index {
                    // Invalid match
                    result += *scores.get(&c).unwrap();
                    break;
                }
            }
        }
    }
    result
}

fn part2(data: &String) -> i64 {
    let mut scores = HashMap::new();
    scores.insert(')', 1);
    scores.insert(']', 2);
    scores.insert('}', 3);
    scores.insert('>', 4);
    let openers = vec!['(', '[', '{', '<'];
    let closers = vec![')', ']', '}', '>'];

    let input = get_lines_as_vec_chars(data);
    let incomplete = input
        .iter()
        .filter(|line| {
            let mut stack: Vec<&char> = Vec::new();
            for c in line.to_owned() {
                if openers.contains(&c) {
                    stack.push(c);
                } else {
                    let last_open = stack.pop().unwrap();
                    let last_open_index = openers.iter().position(|x| *x == *last_open).unwrap();
                    let closer_index = closers.iter().position(|x| *x == *c).unwrap();
                    if last_open_index != closer_index {
                        // Invalid match
                        return false;
                    }
                }
            }
            true
        })
        .collect::<Vec<&Vec<char>>>();
    let mut line_scores: Vec<i64> = Vec::new();
    for line in incomplete {
        let mut stack: Vec<&char> = Vec::new();
        for c in line {
            if openers.contains(&c) {
                stack.push(c);
            } else {
                stack.pop();
            }
        }
        line_scores.push(
            stack
                .iter()
                .map(|c| closers[openers.iter().position(|x| *x == **c).unwrap()])
                .rev()
                .fold(0, |acc, c| (acc * 5) + scores.get(&c).unwrap()),
        );
    }
    line_scores.sort();
    let result = line_scores[line_scores.len() / 2];
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2021_10_ep1() {
        assert_eq!(26397, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_10_ep2() {
        assert_eq!(288957, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_10_rp1() {
        assert_eq!(370407, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_10_rp2() {
        assert_eq!(3249889609, part2(&get_input(YEAR, DAY)));
    }
}
