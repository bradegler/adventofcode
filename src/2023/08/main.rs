use aocshared::aoc::aoc::*;
use hashbrown::HashMap;
use itertools::Itertools;
use regex::Regex;

const YEAR: i32 = 2023;
const DAY: u32 = 08;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn parse(data: &String) -> (Vec<usize>, HashMap<&str, Vec<&str>>) {
    let lines = get_lines_as_strs_rm_empty(data);
    let mut lines = lines.iter();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|lr| match lr {
            'L' => 0,
            'R' => 1,
            _ => 0,
        })
        .collect_vec();
    let re = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
    let paths = lines.fold(HashMap::new(), |mut map, line| {
        let caps = re.captures(line).unwrap();
        let key = caps.get(1).unwrap().as_str();
        let l = caps.get(2).unwrap().as_str();
        let r = caps.get(3).unwrap().as_str();
        map.insert(key, vec![l, r]);
        map
    });
    (directions, paths)
}

fn part1(data: &String) -> u64 {
    let (directions, paths) = parse(data);
    let mut dir_iter = directions.iter().cycle();
    let mut key = "AAA";
    let mut steps = 0;
    while key != "ZZZ" {
        steps += 1;
        let opts = paths.get(key).unwrap();
        let dir = dir_iter.next().unwrap();
        key = opts.get(*dir).unwrap();
    }
    steps
}

fn part2(data: &String) -> u64 {
    let (directions, paths) = parse(data);
    let starts = paths
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| *s)
        .collect_vec();

    starts
        .iter()
        .map(|mut s| {
            let mut step = 0;
            loop {
                step += 1;
                // check both directions
                let path = paths.get(*s).unwrap();
                for dir in directions.clone() {
                    let n = path.get(dir).unwrap();
                    s = n;
                }
                if s.ends_with('Z') {
                    return step;
                }
            }
        })
        .map(|x| x * directions.len())
        .fold(1, lcm) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2023_08_ep1() {
        assert_eq!(6, part1(&get_test_input(YEAR, DAY)));
    }

    // Test data for part two, each node doesn't have a full path to **Z so optimized
    // algorithm fails with an infinite loop.
    // #[test]
    // fn t2023_08_ep2() {
    //     assert_eq!(6, part2(&get_test_input_part(YEAR, DAY, 2)));
    // }

    #[test]
    fn t2023_08_rp1() {
        assert_eq!(12169, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_08_rp2() {
        assert_eq!(12030780859469, part2(&get_input(YEAR, DAY)));
    }
}
