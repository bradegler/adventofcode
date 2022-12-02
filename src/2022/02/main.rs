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

#[derive(PartialEq, Eq, Hash, Debug)]
enum Moves {
    Rock,
    Paper,
    Scissors
}

const WIN: u64 = 6;
const DRAW: u64 = 3;
const LOSE: u64 = 0;

fn part1(data: &String) -> u64 {

    let mut rps = HashMap::new();
    rps.insert('A', Moves::Rock);
    rps.insert('B', Moves::Paper);
    rps.insert('C', Moves::Scissors);
    rps.insert('X', Moves::Rock);
    rps.insert('Y', Moves::Paper);
    rps.insert('Z', Moves::Scissors);

    let turns = get_lines_as_vec_chars(data);
    turns.iter().map(|cs| {
        print!("{:?}", cs);
        let opp = rps.get(&cs[0]).unwrap();
        let me = rps.get(&cs[2]).unwrap();
        score_play(opp, me)
    }).sum()
}

fn score_play(opp: &Moves, me: &Moves) -> u64 {
    println!("Opponent: {:?} - Me: {:?}", opp, me);
    let mut score = HashMap::new();
    score.insert(Moves::Rock, 1);
    score.insert(Moves::Paper,  2);
    score.insert(Moves::Scissors, 3);
    let result = match opp {
        Moves::Rock if me == &Moves::Paper => WIN,
        Moves::Rock if me == &Moves::Scissors => LOSE,
        Moves::Rock if me == &Moves::Rock => DRAW,
        Moves::Paper if me == &Moves::Paper => DRAW,
        Moves::Paper if me == &Moves::Scissors => WIN,
        Moves::Paper if me == &Moves::Rock => LOSE,
        Moves::Scissors if me == &Moves::Paper => LOSE,
        Moves::Scissors if me == &Moves::Scissors => DRAW,
        Moves::Scissors if me == &Moves::Rock => WIN,
        &_ => todo!()
    };
    result + score.get(me).unwrap()
}

fn part2(data: &String) -> u64 {
    let mut rps = HashMap::new();
    rps.insert('A', Moves::Rock);
    rps.insert('B', Moves::Paper);
    rps.insert('C', Moves::Scissors);
    let mut result = HashMap::new();
    result.insert('X', LOSE);
    result.insert('Y', DRAW);
    result.insert('Z', WIN);

    let turns = get_lines_as_vec_chars(data);
    turns.iter().map(|cs| {
        print!("{:?}", cs);
        let opp = rps.get(&cs[0]).unwrap();
        let result = result.get(&cs[2]).unwrap();
        let me = get_for_result(opp, *result);
        score_play(opp, &me)
    }).sum()
}

fn get_for_result(opp: &Moves, result: u64) -> Moves {
    match opp {
        Moves::Rock if result == WIN => Moves::Paper,
        Moves::Rock if result == LOSE => Moves::Scissors,
        Moves::Rock if result == DRAW => Moves::Rock,
        Moves::Paper if result == WIN => Moves::Scissors,
        Moves::Paper if result == LOSE => Moves::Rock,
        Moves::Paper if result == DRAW => Moves::Paper,
        Moves::Scissors if result == WIN => Moves::Rock,
        Moves::Scissors if result == LOSE => Moves::Paper,
        Moves::Scissors if result == DRAW => Moves::Scissors,
        &_ => todo!()
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
