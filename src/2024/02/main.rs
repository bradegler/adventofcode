use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2024;
const DAY: u32 = 02;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let contents = get_lines_as_strs_rm_empty(data);
    contents
        .iter()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|report| is_safe(report))
        .filter(|safe| *safe)
        .count() as u64
}

fn is_safe_dampened(report: Vec<i32>) -> bool {
    if is_safe(report.clone()) {
        return true;
    }
    (0..report.len()).any(|skip| is_safe_report_skip(&report, skip))
}

fn is_safe_report_skip(report: &Vec<i32>, skip: usize) -> bool {
    let descending = if skip == 0 {
        report[1] > report[2]
    } else if skip == 1 {
        report[0] > report[2]
    } else {
        report[0] > report[1]
    };

    report
        .iter()
        .enumerate()
        .filter_map(|(i, value)| if i == skip { None } else { Some(value) })
        .tuple_windows()
        .map(|(&prev, &next)| {
            !((!descending && prev >= next)
                || (descending && prev <= next)
                || prev.abs_diff(next) > 3)
        })
        .filter(|b| !b)
        .count()
        == 0
}

fn is_safe(report: Vec<i32>) -> bool {
    let descending = report[0] > report[1];
    report
        .iter()
        .tuple_windows()
        .map(|(&prev, &next)| {
            !((!descending && prev >= next)
                || (descending && prev <= next)
                || prev.abs_diff(next) > 3)
        })
        .filter(|b| !b)
        .count()
        == 0
}

fn part2(data: &String) -> u64 {
    let contents = get_lines_as_strs_rm_empty(data);
    contents
        .iter()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|report| is_safe_dampened(report))
        .filter(|safe| *safe)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2024_02_ep1() {
        assert_eq!(2, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_02_ep2() {
        assert_eq!(4, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_02_rp1() {
        assert_eq!(670, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_02_rp2() {
        assert_eq!(700, part2(&get_input(YEAR, DAY)));
    }
}
