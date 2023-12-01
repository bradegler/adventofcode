use aocshared::*;
use hashbrown::HashMap;

const YEAR: i32 = 2023;
const DAY: u32 = 01;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let lines = get_lines_as_vec_chars(data);
    let mut sum = 0;
    for line in lines.iter() {
        let left = line.iter().position(|c| c.is_numeric()).unwrap();
        let right = line.iter().rposition(|c| c.is_numeric()).unwrap();
        let value = format!("{}{}", line[left], line[right]);
        let num = value.parse::<u64>().unwrap();
        sum += num;
    }
    sum
}

fn part2(data: &String) -> u64 {
    let lines = get_lines_as_strs(data);
    let words: HashMap<&str, &str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);
    let mut sum = 0;
    for line in lines.iter() {
        let replaced1 = words
            .iter()
            .fold(line.to_string(), |acc, e| acc.replace(e.0, e.1));
        let replaced2 = words
            .iter()
            .fold(line.to_string(), |acc, e| acc.replace(e.0, e.1));
        println!("{}-{}", line, replaced1);
        let left = replaced1
            .chars()
            .into_iter()
            .position(|c| c.is_numeric())
            .unwrap();
        let right = replaced2
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .rposition(|c| c.is_numeric())
            .unwrap();
        println!("{}-{}", left, right);

        let chars = replaced1.chars().collect::<Vec<char>>();
        let value = format!("{}{}", chars[left], chars[right]);
        println!("{}", value);
        let num = value.parse::<u64>().unwrap();
        sum += num;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2023_01_ep1() {
        assert_eq!(142, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_01_ep2() {
        assert_eq!(281, part2(&get_test_input_part(YEAR, DAY, 2)));
    }

    #[test]
    fn t2023_01_rp1() {
        assert_eq!(54304, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_01_rp2() {
        assert_eq!(54418, part2(&get_input(YEAR, DAY)));
    }
}
