use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2024;
const DAY: u32 = 05;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let (rules, pagesets) = parse(data);
    pagesets
        .iter()
        .filter(|ps| is_valid(ps, &rules))
        .map(|ps| ps[ps.len() / 2] as u64)
        .sum()
}

fn part2(data: &String) -> u64 {
    let (rules, pagesets) = parse(data);
    pagesets
        .iter()
        .filter(|ps| !is_valid(ps, &rules))
        .map(|ps| order(ps, &rules))
        .map(|ps| ps[ps.len() / 2] as u64)
        .sum()
}

fn parse(data: &String) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let lines = get_lines_as_strs_rm_empty(data);
    let mut rules = vec![vec![]; 100];
    let mut pagesets: Vec<Vec<usize>> = vec![];
    for l in lines {
        match l {
            rule if rule.contains("|") => {
                let parts = rule
                    .split("|")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec();
                if parts.len() != 2 {
                    panic!("Invalid rule {:?}", rule);
                }
                let key = parts[0];
                let val = parts[1];
                rules[key].push(val);
            }
            pages if pages.contains(",") => {
                let parts = pages
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec();
                pagesets.push(parts);
            }
            _ => {}
        };
    }
    (rules, pagesets)
}

fn order(pageset: &Vec<usize>, rules: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut new_pageset = pageset.clone();
    while !is_valid(&new_pageset, rules) {
        for page in pageset {
            let cur_page_loc = new_pageset.iter().position(|p| p == page).unwrap();
            let page_rules = &rules[*page];
            for pr_page in page_rules {
                if new_pageset[0..cur_page_loc].contains(pr_page) {
                    new_pageset.remove(cur_page_loc);
                    new_pageset.insert(0, *page);
                    break;
                }
            }
        }
    }
    new_pageset.clone()
}

fn is_valid(pageset: &Vec<usize>, rules: &Vec<Vec<usize>>) -> bool {
    for (idx, page) in pageset.iter().enumerate() {
        let page_rules = &rules[*page];
        for pr_page in page_rules {
            if pageset[0..idx].contains(pr_page) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2024_05_ep1() {
        assert_eq!(143, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_05_ep2() {
        assert_eq!(123, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_05_rp1() {
        assert_eq!(4959, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_05_rp2() {
        assert_eq!(4655, part2(&get_input(YEAR, DAY)));
    }
}
