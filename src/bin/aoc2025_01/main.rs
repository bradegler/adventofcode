use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2025;
const DAY: u32 = 01;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

struct Dial {
    start: i32,
    end: i32,
    pos: i32,
}

impl Dial {
    fn new(start: i32, end: i32, pos: i32) -> Dial {
        Dial {
            start,
            end,
            pos: pos,
        }
    }

    fn rotate(&mut self, rotation: i32) {
        self.pos = (self.pos + rotation) % (self.end - self.start + 1);
        if self.pos < self.start {
            self.pos += self.end - self.start + 1;
        }
    }
}

fn part1(data: &String) -> u64 {
    let input = get_lines_as_strs_rm_empty(data)
        .into_iter()
        .map(|s| s.replace("L", "-").replace("R", ""))
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec();

    let mut dial = Dial::new(0, 99, 50);
    input
        .into_iter()
        .map(|rotation| {
            dial.rotate(rotation);
            dial.pos
        })
        .filter(|p| *p == 0)
        .count() as u64
}

fn part2(data: &String) -> u64 {
    let input = get_lines_as_strs_rm_empty(data)
        .into_iter()
        .map(|s| s.replace("L", "-").replace("R", ""))
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec();

    let mut dial = Dial::new(0, 99, 50);
    input
        .into_iter()
        .map(|rotation| {
            let mut stops = 0;
            for _ in 0..rotation.abs() {
                dial.rotate(rotation.signum());
                if dial.pos == 0 {
                    stops += 1;
                }
            }
            stops
        })
        .sum::<i32>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_01_ep1() {
        assert_eq!(3, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_01_ep2() {
        assert_eq!(6, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_01_rp1() {
        assert_eq!(1007, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_01_rp2() {
        assert_eq!(5820, part2(&get_input(YEAR, DAY)));
    }
}
