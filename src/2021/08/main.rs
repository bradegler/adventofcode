use aocshared::*;
use itertools::Itertools;

const YEAR: i32 = 2021;
const DAY: u32 = 08;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    let input = get_lines_as_strs(data);
    let result = input.iter().fold(0, |sum, line| {
        let mut parts = line.split(" | ");
        let patterns = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
        let count_lengths = patterns.iter().fold(vec![0; 7], |mut acc, s| {
            acc[s.len() - 1] += 1;
            acc
        });
        let segments = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
        sum + segments
            .iter()
            .filter(|s| count_lengths[s.len() - 1] == 1)
            .count() as i32
    });
    result
}

fn part2(data: &String) -> i32 {
    let input = get_lines_as_strs(data)
        .iter()
        .map(|l| {
            let (a, b) = l.split_once(" | ").unwrap();
            let x = a.split_whitespace().collect::<Vec<_>>();
            let y = b.split_whitespace().collect::<Vec<_>>();
            (x, y)
        })
        .collect::<Vec<_>>();
    let result = input
        .iter()
        .map(|d| {
            "abcdefg"
                .chars()
                .permutations(7)
                .find_map(|perm| try_permutation(&perm, d))
                .unwrap()
        })
        .sum::<usize>();
    result.try_into().unwrap()
}

fn display_digit(perm: &[char], s: &str) -> Option<usize> {
    let decoded = s
        .chars()
        .map(|c| perm[(c as u8 - b'a') as usize])
        .sorted()
        .collect::<String>();
    let digit = match decoded.as_str() {
        "abcdefg" => 8,
        "bcdef" => 5,
        "acdfg" => 2,
        "abcdf" => 3,
        "abd" => 7,
        "abcdef" => 9,
        "bcdefg" => 6,
        "abef" => 4,
        "abcdeg" => 0,
        "ab" => 1,
        _ => return None,
    };
    Some(digit)
}

fn try_permutation(perm: &[char], (a, b): &(Vec<&str>, Vec<&str>)) -> Option<usize> {
    let invalid = a
        .iter()
        .map(|s| display_digit(&perm, s))
        .any(|o| o.is_none());
    if invalid {
        return None;
    }

    let ans = b
        .iter()
        .rev()
        .enumerate()
        .map(|(i, s)| display_digit(&perm, s).unwrap() * 10usize.pow(i as u32))
        .sum();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_08_ep1() {
        assert_eq!(26, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_08_ep2() {
        assert_eq!(61229, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_08_rp1() {
        assert_eq!(237, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_08_rp2() {
        assert_eq!(1009098, part2(&get_input(YEAR, DAY)));
    }
}
