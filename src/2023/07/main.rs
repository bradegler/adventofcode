use std::cmp::Ordering;

use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2023;
const DAY: u32 = 07;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn card_ranks(c: char, jokers: bool) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if jokers {
                0
            } else {
                11
            }
        }
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => unreachable!(),
    }
}

fn part1(data: &String) -> u64 {
    let lines = get_lines_as_strs_rm_empty(data);
    let hands = lines.iter().map(Hand::parse);
    let rank = Hand::rank;
    let jokers = false;
    hands
        .sorted_by(|h1, h2| {
            let r1 = rank(h1);
            let r2 = rank(h2);
            if r1 == r2 {
                for cidx in 0..5 {
                    let l1 = card_ranks(h1.cards.chars().nth(cidx).unwrap(), jokers);
                    let l2 = card_ranks(h2.cards.chars().nth(cidx).unwrap(), jokers);
                    if l1 > l2 {
                        return Ordering::Greater;
                    } else if l1 < l2 {
                        return Ordering::Less;
                    }
                }
                return Ordering::Equal;
            } else {
                return r1.cmp(&r2);
            }
        })
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum()
}

fn part2(data: &String) -> u64 {
    let lines = get_lines_as_strs_rm_empty(data);
    let hands = lines.iter().map(Hand::parse);
    let rank = Hand::rank_with_jokers;
    let jokers = true;
    hands
        .sorted_by(|h1, h2| {
            let r1 = rank(h1);
            let r2 = rank(h2);
            if r1 == r2 {
                for cidx in 0..5 {
                    let l1 = card_ranks(h1.cards.chars().nth(cidx).unwrap(), jokers);
                    let l2 = card_ranks(h2.cards.chars().nth(cidx).unwrap(), jokers);
                    if l1 > l2 {
                        return Ordering::Greater;
                    } else if l1 < l2 {
                        return Ordering::Less;
                    }
                }
                return Ordering::Equal;
            } else {
                return r1.cmp(&r2);
            }
        })
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum()
}
#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u64,
}

impl Hand {
    fn parse(line: &&str) -> Self {
        let (cards, bid) = line.split_once(" ").unwrap();
        Self {
            cards: cards.to_string(),
            bid: bid.parse::<u64>().unwrap(),
        }
    }
    fn rank_with_jokers(&self) -> usize {
        let counts_by_card = self.cards.chars().counts();
        let counts = counts_by_card
            .iter()
            .filter(|&(&k, _)| k != 'J')
            .map(|(_, &v)| v)
            .collect::<Vec<_>>();
        let jokers = *counts_by_card.get(&'J').unwrap_or(&0);
        self.get_hand_type(&counts, jokers)
    }

    fn get_hand_type(&self, counts: &[usize], jokers: usize) -> usize {
        match (*counts.iter().max().unwrap_or(&0), jokers) {
            (a, b) if a + b == 5 => 6,
            (a, b) if a + b == 4 => 5,
            (3, 0) => {
                if counts.contains(&2) {
                    4
                } else {
                    3
                }
            }
            (2, _) => {
                let pairs = counts.iter().filter(|&&v| v == 2).count();
                match (pairs, jokers) {
                    (2, 1) => 4,
                    (1, 1) => 3,
                    (2, 0) => 2,
                    _ => 1,
                }
            }
            (1, 2) => 3,
            (1, 1) => 1,
            _ => 0,
        }
    }

    fn rank(&self) -> usize {
        let counts = self.cards.chars().counts();
        let counts = counts.iter().map(|(_, &v)| v).collect::<Vec<_>>();
        self.get_hand_type(&counts, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2023_07_ep1() {
        assert_eq!(6440, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_07_ep2() {
        assert_eq!(5905, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_07_rp1() {
        assert_eq!(253910319, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_07_rp2() {
        assert_eq!(254083736, part2(&get_input(YEAR, DAY)));
    }
}
