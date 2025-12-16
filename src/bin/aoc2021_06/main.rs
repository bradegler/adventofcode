use aocshared::aoc::aoc::*;

const YEAR: i32 = 2021;
const DAY: u32 = 06;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn spawn_count(input: &str, days: i32) -> i64 {
    let start = get_numbers_from_line(input);
    let mut fish_days = start.iter().fold(vec![0; 9], |mut acc, x| {
        acc[*x as usize] += 1;
        acc
    });
    for _ in 1..=days {
        fish_days = (0..8).fold(vec![0; 9], |mut acc, x| {
            if x == 0 {
                acc[6] = fish_days[x];
                acc[8] = fish_days[x];
            }
            acc[x] = acc[x] + fish_days[x + 1];
            acc
        });
    }
    fish_days.iter().sum::<i64>()
}

fn part1(data: &String) -> i64 {
    spawn_count(data, 80)
}

fn part2(data: &String) -> i64 {
    spawn_count(data, 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2021_06_ep1() {
        assert_eq!(5934, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_06_ep2() {
        assert_eq!(26984457539, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_06_rp1() {
        assert_eq!(395627, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_06_rp2() {
        assert_eq!(1767323539209, part2(&get_input(YEAR, DAY)));
    }
}
