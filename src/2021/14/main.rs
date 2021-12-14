use aocshared::*;
use regex::Regex;
use std::collections::HashMap;

const YEAR: i32 = 2021;
const DAY: u32 = 14;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> usize {
    simulation(data, 10)
}

fn freq_map(data: &str) -> HashMap<(char, char), usize> {
    let mut chars = data.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.dedup();
    // Create a map of all the pairs of chars
    let len = chars.len();
    (0..len)
        .map(|i| {
            let ci = &chars[i];
            (0..len).map(|j| ((*ci, chars[j]), 0))
        })
        .flatten()
        .collect::<HashMap<_, _>>()
}

fn part2(data: &String) -> usize {
    simulation(data, 40)
}

fn simulation(data: &String, iterations: u32) -> usize {
    let lines = get_lines_as_strs_rm_empty(data);
    let polymer = lines[0];
    let re = Regex::new(r"(\w)(\w) -> (\w)").unwrap();
    let conversions = lines[1..]
        .iter()
        .map(|l| re.captures(l).unwrap())
        .map(|captures| {
            (
                (
                    captures
                        .get(1)
                        .map(|c| c.as_str().chars().next().unwrap())
                        .unwrap(),
                    captures
                        .get(2)
                        .map(|c| c.as_str().chars().next().unwrap())
                        .unwrap(),
                ),
                captures
                    .get(3)
                    .map(|c| c.as_str().chars().next().unwrap())
                    .unwrap(),
            )
        })
        .collect::<HashMap<(_, _), _>>();
    let mut freq = freq_map(data);
    // Seed the polymer frequency values
    polymer
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .for_each(|w| *freq.get_mut(&(w[0], w[1])).unwrap() += 1);

    let mut new_freq = freq.clone();
    for _ in 0..iterations {
        new_freq.iter_mut().for_each(|(_, v)| *v = 0);
        for (k, v) in freq.iter() {
            if let Some(&c) = conversions.get(k) {
                *new_freq.get_mut(&(k.0, c)).unwrap() += v;
                *new_freq.get_mut(&(c, k.1)).unwrap() += v;
            } else {
                *new_freq.get_mut(k).unwrap() += v;
            }
        }
        std::mem::swap(&mut freq, &mut new_freq);
    }

    let mut f = HashMap::new();
    for (k, v) in freq {
        *f.entry(k.0).or_insert(0) += v;
        *f.entry(k.1).or_insert(0) += v;
    }
    let mut vals = f
        .into_iter()
        .map(|(_, v)| if v & 1 == 1 { v / 2 + 1 } else { v / 2 })
        .filter(|v| *v != 0)
        .collect::<Vec<_>>();
    vals.sort_unstable();
    vals[vals.len() - 1] - vals[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_14_ep1() {
        assert_eq!(1588, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_14_ep2() {
        assert_eq!(2188189693529, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_14_rp1() {
        assert_eq!(2967, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_14_rp2() {
        assert_eq!(3692219987038, part2(&get_input(YEAR, DAY)));
    }
}
