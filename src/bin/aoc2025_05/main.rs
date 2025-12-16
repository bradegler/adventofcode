use aocshared::aoc::aoc::*;

const YEAR: i32 = 2025;
const DAY: u32 = 05;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let (ranges, ids) =
        get_lines_as_strs(data)
            .into_iter()
            .fold((vec![], vec![]), |mut parts, line| {
                if line.contains("-") {
                    let splits: Vec<&str> = line.split("-").collect();
                    let start = splits[0].parse::<usize>().unwrap();
                    let end = splits[1].parse::<usize>().unwrap();
                    parts.0.push((start, end));
                } else {
                    if line.len() > 0 {
                        let val = line.parse::<usize>().unwrap();
                        parts.1.push(val)
                    }
                }
                parts
            });
    let mut count = 0;
    for id in ids {
        for rng in ranges.clone().into_iter() {
            if id >= rng.0 && id <= rng.1 {
                count += 1;
                break;
            }
        }
    }
    count
}

fn part2(data: &String) -> u64 {
    let mut ranges = get_lines_as_strs(data)
        .into_iter()
        .fold(vec![], |mut parts, line| {
            if line.contains("-") {
                let splits: Vec<&str> = line.split("-").collect();
                let start = splits[0].parse::<usize>().unwrap();
                let end = splits[1].parse::<usize>().unwrap();
                parts.push((start, end));
            }
            parts
        });

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_ranges: Vec<(usize, usize)> = Vec::new();
    for range in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges
        .iter()
        .map(|(start, end)| (end - start + 1) as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_05_ep1() {
        assert_eq!(3, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_05_ep2() {
        assert_eq!(14, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_05_rp1() {
        assert_eq!(828, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_05_rp2() {
        assert_eq!(352681648086146, part2(&get_input(YEAR, DAY)));
    }
}
