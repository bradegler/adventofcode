#![feature(extract_if)]
use aocshared::*;

const YEAR: i32 = 2021;
const DAY: u32 = 4;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

struct Board {
    grid: Vec<Vec<u32>>,
    state: Vec<Vec<bool>>,
}

impl Board {
    fn is_winner(&self) -> bool {
        self.state.iter().any(|row| row.iter().all(|&b| b))
            || transpose(&self.state, false)
                .iter()
                .any(|row| row.iter().all(|&b| b))
    }

    fn call(&mut self, val: u32) {
        for y in 0..5 {
            for x in 0..5 {
                if self.grid[y][x] == val {
                    self.state[y][x] = true;
                }
            }
        }
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !self.state[y][x] {
                    score += self.grid[y][x];
                }
            }
        }
        return score;
    }
}

fn part1(data: &String) -> u32 {
    let input = get_lines_as_strs(data);
    let call_order = input[0]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut boards = chunks(&input[1..], 6)
        .map(|f| {
            f.iter()
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|f| Board {
            grid: f,
            state: vec![vec![false; 5]; 5],
        })
        .collect::<Vec<_>>();

    let mut winner_call = 0;
    let mut winner: Option<&Board> = None;
    for call in call_order {
        boards.iter_mut().for_each(|b| b.call(call));
        winner = boards.iter().filter(|b| b.is_winner()).next();
        if winner.is_some() {
            winner_call = call;
            break;
        }
    }
    let score = winner.unwrap().score();
    score * winner_call
}

fn part2(data: &String) -> u32 {
    let input = get_lines_as_strs(data);
    let call_order = input[0]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    println!("Call order: {:?}", call_order);
    let boards = chunks(&input[1..], 6)
        .map(|f| {
            f.iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.split_whitespace())
                .map(|s| s.collect::<Vec<&str>>())
                .map(|s| {
                    s.iter()
                        .map(|v| v.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|f| Board {
            grid: f,
            state: vec![vec![false; 5]; 5],
        })
        .collect::<Vec<_>>();

    let mut winner_call = 0;
    let mut winner: Option<&mut Board> = None;
    let mut remaining = boards;
    for call in call_order {
        remaining.iter_mut().for_each(|b| b.call(call));
        let r = remaining
            .extract_if(|b| !b.is_winner())
            .collect::<Vec<Board>>();
        if r.len() == 0 {
            winner_call = call;
            winner = remaining.iter_mut().next();
            break;
        }
        remaining = r;
    }
    let winboard = winner.unwrap();
    println!("Winner call: {}", winner_call);
    println!("Board: {:?}", winboard.state);
    let score = winboard.score();
    println!("Score: {}", score);

    let result = score * winner_call;
    println!("Part 2 Result: {}", result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_04_ep1() {
        assert_eq!(4512, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_04_ep2() {
        assert_eq!(1924, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_04_rp1() {
        assert_eq!(31424, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_04_rp2() {
        assert_eq!(23042, part2(&get_input(YEAR, DAY)));
    }
}
