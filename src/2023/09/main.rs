use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2023;
const DAY: u32 = 09;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn parse(data: &String) -> Vec<Vec<i32>> {
    get_lines_as_strs(data)
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn part1(data: &String) -> u64 {
    let histories = parse(data);
    histories.iter().fold(0, |acc, history| {
        acc + predict_next(history) + history.last().unwrap()
    }) as u64
}

fn next_seq(history: &Vec<i32>) -> Vec<i32> {
    history
        .windows(2)
        .map(|win| if win.len() > 1 { win[1] - win[0] } else { 0 })
        .collect_vec()
}

fn predict_next(history: &Vec<i32>) -> i32 {
    if history.iter().sum::<i32>() == 0 {
        return 0;
    }
    let seq = next_seq(history);
    seq.last().unwrap() + predict_next(&seq)
}

fn part2(data: &String) -> u64 {
    let histories = parse(data);
    histories
        .iter()
        .map(|history| {
            let mut seqs = vec![history.to_owned()];
            let mut seq = next_seq(history);
            seqs.push(seq.clone());
            let mut sum = seq.iter().sum::<i32>();
            while sum != 0 {
                let next = next_seq(&seq.clone());
                seqs.push(next.clone());
                sum = next.iter().sum::<i32>();
                seq = next;
            }
            seqs.iter()
                .rev()
                .fold(0, |acc: i32, history| history[0] - acc)
        })
        .sum::<i32>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2023_09_ep1() {
        assert_eq!(114, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_09_ep2() {
        assert_eq!(2, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_09_rp1() {
        assert_eq!(1647269739, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_09_rp2() {
        assert_eq!(864, part2(&get_input(YEAR, DAY)));
    }
}
