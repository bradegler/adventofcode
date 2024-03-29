use aocshared::aoc::aoc::*;
use regex::Regex;
use std::collections::HashMap;

const YEAR: i32 = 2020;
const DAY: u32 = 07;
const TARGET_BAG: &str = "shiny gold";

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn can_hold_bag(bags: &Vec<(i32, String)>, all: &HashMap<String, Vec<(i32, String)>>) -> bool {
    bags.iter()
        .any(|(_, name)| *name == TARGET_BAG || can_hold_bag(all.get(name).unwrap(), all))
}

fn part1(data: &String) -> usize {
    let recontainer = Regex::new(r"([a-z ]+) bags contain (.*)\.$").unwrap();
    let recontents = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();
    let input = data
        .lines()
        .map(|x| recontainer.captures(x).unwrap())
        .map(|c| (c[1].to_string(), c[2].to_string()))
        .fold(HashMap::new(), |acc, (b, c)| {
            let mut m = acc;
            if c == "no other bags.".to_string() {
                m.insert(b, vec![]);
            } else {
                m.insert(
                    b,
                    recontents
                        .captures_iter(&c)
                        .map(|c| (c[1].parse::<i32>().unwrap(), c[2].to_string()))
                        .collect(),
                );
            }
            m
        });
    input
        .iter()
        .filter(|(_, v)| can_hold_bag(v, &input))
        .count()
}

fn sum_bags(bag_name: &str, bags: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    bags.get(bag_name)
        .unwrap()
        .iter()
        .map(|(count, bag)| count + count * sum_bags(bag, bags))
        .sum()
}

fn part2(data: &String) -> i32 {
    let recontainer = Regex::new(r"([a-z ]+) bags contain (.*)\.$").unwrap();
    let recontents = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();
    let input = data
        .lines()
        .map(|x| recontainer.captures(x).unwrap())
        .map(|c| (c[1].to_string(), c[2].to_string()))
        .fold(HashMap::new(), |acc, (b, c)| {
            let mut m = acc;
            if c == "no other bags.".to_string() {
                m.insert(b, vec![]);
            } else {
                m.insert(
                    b,
                    recontents
                        .captures_iter(&c)
                        .map(|c| (c[1].parse::<i32>().unwrap(), c[2].to_string()))
                        .collect(),
                );
            }
            m
        });
    sum_bags(TARGET_BAG, &input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2020_07_ep1() {
        assert_eq!(4, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_07_ep2() {
        assert_eq!(32, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_07_rp1() {
        assert_eq!(128, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_07_rp2() {
        assert_eq!(20189, part2(&get_input(YEAR, DAY)));
    }
}
