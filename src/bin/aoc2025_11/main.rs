use aocshared::aoc::aoc::*;
use hashbrown::HashMap;
use itertools::Itertools;

const YEAR: i32 = 2025;
const DAY: u32 = 11;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn solve1<'a>(devices: &HashMap<&'a str, Vec<&'a str>>, start: &'a str, end: &str) -> u64 {
    if start == end {
        return 1;
    }
    match devices.get(start) {
        Some(outputs) => outputs.iter().map(|d| solve1(devices, d, end)).sum(),
        None => 0,
    }
}

fn solve2<'a>(
    devices: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    end: &str,
    has_fft: bool,
    has_dac: bool,
    memo: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    if current == end {
        if has_fft && has_dac {
            return 1;
        }
        return 0;
    }

    let key = (current, has_fft, has_dac);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    let count = match devices.get(current) {
        Some(outputs) => outputs
            .iter()
            .map(|d| {
                let next_fft = has_fft || *d == "fft";
                let next_dac = has_dac || *d == "dac";
                solve2(devices, d, end, next_fft, next_dac, memo)
            })
            .sum(),
        None => 0,
    };

    memo.insert(key, count);
    count
}

fn part1(data: &String) -> u64 {
    let devices = get_lines_as_strs_rm_empty(data)
        .iter()
        .fold(HashMap::new(), |mut acc, s| {
            let (device, outputs) = s.split_once(":").unwrap();
            let outputs = outputs.split(" ").filter(|s| s.len() > 0).collect_vec();
            acc.insert(device, outputs);

            acc
        });
    let start = "you";
    let end = "out";
    solve1(&devices, start, end)
}

fn part2(data: &String) -> u64 {
    let devices = get_lines_as_strs_rm_empty(data)
        .iter()
        .fold(HashMap::new(), |mut acc, s| {
            let (device, outputs) = s.split_once(":").unwrap();
            let outputs = outputs.split(" ").filter(|s| s.len() > 0).collect_vec();
            acc.insert(device, outputs);

            acc
        });
    let start = "svr";
    let end = "out";
    let mut memo = HashMap::new();
    solve2(&devices, start, end, false, false, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_11_ep1() {
        assert_eq!(5, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_11_ep2() {
        assert_eq!(2, part2(&get_test_input_part(YEAR, DAY, 2)));
    }

    #[test]
    fn t2025_11_rp1() {
        assert_eq!(786, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_11_rp2() {
        assert_eq!(495845045016588, part2(&get_input(YEAR, DAY)));
    }
}
