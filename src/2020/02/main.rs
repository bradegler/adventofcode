use aocshared::*;
use regex::Regex;

const YEAR: i32 = 2020;
const DAY: u32 = 02;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> usize {
    let lines = get_lines_as_strs(data);
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    lines
        .iter()
        .map(|s| {
            let caps = re.captures(s).unwrap();
            let min = caps[1].parse::<i32>().unwrap();
            let max = caps[2].parse::<i32>().unwrap();
            let letter = caps[3].chars().next().unwrap();
            let password = caps[4].chars().collect::<Vec<char>>();
            println!(
                "{} {} {} {}",
                min,
                max,
                letter,
                String::from_iter(&password)
            );
            (min, max, letter, password)
        })
        .filter(|(min, max, letter, password)| {
            let cnt: i32 = password
                .iter()
                .filter(|c| c == &letter)
                .count()
                .try_into()
                .unwrap();
            &cnt >= min && &cnt <= max
        })
        .count()
}

fn part2(data: &String) -> usize {
    let lines = get_lines_as_strs(data);
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    lines
        .iter()
        .map(|s| {
            let caps = re.captures(s).unwrap();
            let min = caps[1].parse::<i32>().unwrap();
            let max = caps[2].parse::<i32>().unwrap();
            let letter = caps[3].chars().next().unwrap();
            let password = caps[4].chars().collect::<Vec<char>>();
            println!(
                "{} {} {} {}",
                min,
                max,
                letter,
                String::from_iter(&password)
            );
            (min, max, letter, password)
        })
        .filter(|(min, max, letter, password)| {
            let at_min = &password[(min - 1) as usize] == letter;
            let at_max = &password[(max - 1) as usize] == letter;
            at_min ^ at_max
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2020_02_ep1() {
        assert_eq!(2, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_02_ep2() {
        assert_eq!(1, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_02_rp1() {
        assert_eq!(622, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_02_rp2() {
        assert_eq!(263, part2(&get_input(YEAR, DAY)));
    }
}
