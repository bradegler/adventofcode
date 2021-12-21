use aocshared::*;
use std::collections::VecDeque;

const YEAR: i32 = 2021;
const DAY: u32 = 21;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    let starts = get_lines_as_strs(data)
        .iter()
        .map(|s| s.split_once(": ").unwrap().1.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("{:?}", starts);
    let mut rolls: i32 = 0;
    let mut scores: Vec<i32> = vec![0; starts.len()];
    let mut positions = vec![(1..=10).collect::<VecDeque<usize>>(); starts.len()];
    positions.iter_mut().enumerate().for_each(|(i, p)| {
        p.rotate_left((starts[i] - 1) as usize);
    });
    println!("{:?}", positions[0]);
    println!("{:?}", positions[1]);

    let mut die: VecDeque<usize> = (1..=100).collect::<VecDeque<usize>>();
    let mut winner = false;
    let mut turn = 0;
    while !winner {
        turn += 1;
        positions.iter_mut().enumerate().for_each(|(i, p)| {
            if !winner {
                let moves = die.iter().take(3).sum::<usize>();
                die.rotate_left(3);
                for _ in 1..(moves / 10) {
                    p.rotate_left(10);
                }
                p.rotate_left(moves % 10);
                scores[i] += *p.front().unwrap() as i32;
                rolls += 3;
                if scores[i] >= 1000 {
                    winner = true;
                }
            }
        });
        println!("Turn {} - Scores {:?}", turn, scores);
    }
    println!("rolls: {}", rolls);
    scores.iter().min().unwrap() * rolls
}

fn part2(data: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_21_ep1() {
        assert_eq!(739785, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_21_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_21_rp1() {
        assert_eq!(678468, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_21_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
