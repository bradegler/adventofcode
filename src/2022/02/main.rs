use aocshared::*;
use hashbrown::HashMap;

const YEAR: i32 = 2022;
const DAY: u32 = 02;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

const ROCK: u64 = 1;
const PAPER: u64 = 2;
const SCISSORS: u64 = 3;

const WIN: u64 = 6;
const DRAW: u64 = 3;
const LOSE: u64 = 0;

fn part1(data: &String) -> u64 {
    let rps = HashMap::<char, u64>::from([
        ('A', ROCK),
        ('B', PAPER),
        ('C', SCISSORS),
        ('X', ROCK),
        ('Y', PAPER),
        ('Z', SCISSORS),
    ]);

    get_lines_as_vec_chars(data)
        .iter()
        .map(|cs| {
            (match (*rps.get(&cs[0]).unwrap(), *rps.get(&cs[2]).unwrap()) {
                (ROCK, PAPER) | (PAPER, SCISSORS) | (SCISSORS, ROCK) => WIN,
                (ROCK, SCISSORS) | (PAPER, ROCK) | (SCISSORS, PAPER) => LOSE,
                (ROCK, ROCK) | (PAPER, PAPER) | (SCISSORS, SCISSORS) => DRAW,
                _ => todo!(),
            }) + *rps.get(&cs[2]).unwrap()
        })
        .sum()
}

fn part2(data: &String) -> u64 {
    let rps = HashMap::<char, u64>::from([
        ('A', ROCK),
        ('B', PAPER),
        ('C', SCISSORS),
        ('X', LOSE),
        ('Y', DRAW),
        ('Z', WIN),
    ]);

    get_lines_as_vec_chars(data)
        .iter()
        .map(|cs| {
            (match (*rps.get(&cs[0]).unwrap(), *rps.get(&cs[2]).unwrap()) {
                (ROCK, WIN) | (PAPER, DRAW) | (SCISSORS, LOSE) => PAPER,
                (ROCK, LOSE) | (PAPER, WIN) | (SCISSORS, DRAW) => SCISSORS,
                (ROCK, DRAW) | (PAPER, LOSE) | (SCISSORS, WIN) => ROCK,
                _ => todo!(),
            }) + *rps.get(&cs[2]).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2022_02_ep1() {
        assert_eq!(15, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_02_ep2() {
        assert_eq!(12, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_02_rp1() {
        assert_eq!(10816, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_02_rp2() {
        assert_eq!(11657, part2(&get_input(YEAR, DAY)));
    }
}
