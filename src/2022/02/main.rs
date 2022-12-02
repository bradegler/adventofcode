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

    let mut rps = HashMap::new();
    rps.insert('A', ROCK);
    rps.insert('B', PAPER);
    rps.insert('C', SCISSORS);
    rps.insert('X', ROCK);
    rps.insert('Y', PAPER);
    rps.insert('Z', SCISSORS);

    get_lines_as_vec_chars(data).iter().map(|cs| 
        score_play(*rps.get(&cs[0]).unwrap(), *rps.get(&cs[2]).unwrap())
    ).sum()
}

fn score_play(opp: u64, me: u64) -> u64 {
    (match opp {
        ROCK if me == PAPER=> WIN,
        ROCK if me == SCISSORS=> LOSE,
        ROCK if me == ROCK=> DRAW,
        PAPER if me == PAPER => DRAW,
        PAPER if me == SCISSORS => WIN,
        PAPER if me == ROCK => LOSE,
        SCISSORS if me == PAPER => LOSE,
        SCISSORS if me == SCISSORS => DRAW,
        SCISSORS if me == ROCK => WIN,
        _ => todo!()
    }) + me
}

fn part2(data: &String) -> u64 {
    let mut rps = HashMap::new();
    rps.insert('A', ROCK);
    rps.insert('B', PAPER);
    rps.insert('C', SCISSORS);
    let mut result = HashMap::new();
    result.insert('X', LOSE);
    result.insert('Y', DRAW);
    result.insert('Z', WIN);

    let turns = get_lines_as_vec_chars(data);
    turns.iter().map(|cs| {
        let opp = rps.get(&cs[0]).unwrap();
        let result = result.get(&cs[2]).unwrap();
        let me = get_for_result(*opp, *result);
        score_play(*opp, me)
    }).sum()
}

fn get_for_result(opp: u64, result: u64) -> u64 {
    match opp {
        ROCK if result == WIN => PAPER,
        ROCK if result == LOSE => SCISSORS,
        ROCK if result == DRAW => ROCK,
        PAPER if result == WIN => SCISSORS,
        PAPER if result == LOSE => ROCK,
        PAPER if result == DRAW => PAPER,
        SCISSORS if result == WIN => ROCK,
        SCISSORS if result == LOSE => PAPER,
        SCISSORS if result == DRAW => SCISSORS,
        _ => todo!()
    }
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
