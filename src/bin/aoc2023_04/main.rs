use aocshared::aoc::aoc::*;
use hashbrown::HashMap;
use regex::Regex;

const YEAR: i32 = 2023;
const DAY: u32 = 04;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let re = Regex::new(r"Card .*:").unwrap();
    let lines = get_lines_as_strs(data)
        .into_iter()
        .map(|l| re.replace_all(l, "").replace("  ", " "));
    lines
        .fold(vec![], |mut acc: Vec<Vec<Vec<u64>>>, line| {
            let parts = line
                .split("|")
                .map(|side| {
                    side.trim()
                        .split(" ")
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>();
            acc.push(parts);
            acc
        })
        .iter()
        .fold(0, |acc, card| {
            let win_numbers = card.get(0).unwrap();
            let card_numbers = card.get(1).unwrap();
            let cnt = card_numbers
                .iter()
                .filter(|c| win_numbers.contains(c))
                .count();
            if cnt == 0 {
                return acc;
            }
            acc + u64::pow(2, (cnt - 1) as u32)
        })
}

fn part2(data: &String) -> u64 {
    let re = Regex::new(r"Card .*:").unwrap();
    let lines = get_lines_as_strs(data)
        .into_iter()
        .map(|l| re.replace_all(l, "").replace("  ", " "));
    let parsed = lines.fold(vec![], |mut acc: Vec<Vec<Vec<u64>>>, line| {
        let parts = line
            .split("|")
            .map(|side| {
                side.trim()
                    .split(" ")
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();
        acc.push(parts);
        acc
    });
    let winnings = parsed.iter().map(|card| {
        let win_numbers = card.get(0).unwrap();
        let card_numbers = card.get(1).unwrap();
        card_numbers
            .iter()
            .filter(|c| win_numbers.contains(c))
            .count() as usize
    });
    let mut counts = HashMap::<usize, u32>::new();
    for (id, card_count) in winnings.enumerate() {
        println!("{} - {}", id, card_count);
        let value = counts.entry(id).and_modify(|c| *c += 1).or_insert(1);
        println!("{}", value);
        if card_count != 0 {
            for _ in 0..*value {
                for i in (id + 1)..(id + card_count + 1) {
                    counts.entry(i).and_modify(|c| *c += 1).or_insert(1);
                }
            }
        }
    }
    counts.values().sum::<u32>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2023_04_ep1() {
        assert_eq!(13, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_04_ep2() {
        assert_eq!(30, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_04_rp1() {
        assert_eq!(15268, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_04_rp2() {
        assert_eq!(6283755, part2(&get_input(YEAR, DAY)));
    }
}
