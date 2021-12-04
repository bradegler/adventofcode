#![feature(drain_filter)]

use aocshared::chunks;
use aocshared::get_input;
use grid::Grid;

const YEAR: i32 = 2021;
const DAY: u32 = 4;

fn main() {
    let i = get_input(YEAR, DAY);
    part1(&i);
    part2(&i);
}

struct Board {
    grid: Vec<Vec<i32>>,
    state: Grid<bool>,
}

impl Board {
    fn is_winner(&self) -> bool {
        for row in 0..self.state.rows() {
            if self.state.iter_row(row).all(|&x| x) {
                return true;
            }
        }
        for col in 0..self.state.cols() {
            if self.state.iter_col(col).all(|&x| x) {
                return true;
            }
        }
        false
    }

    fn call(&mut self, val: i32) {
        for y in 0..5 {
            for x in 0..5 {
                if self.grid[y][x] == val {
                    self.state[y][x] = true;
                }
            }
        }
    }

    fn score(&self) -> i32 {
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

fn part1(data: &String) -> i32 {
    println!("Part 1");
    let input = data.split("\n").collect::<Vec<&str>>();
    let call_order = input[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!("Call order: {:?}", call_order);
    let mut boards = chunks(&input[1..], 6)
        .map(|f| {
            f.iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.split_whitespace())
                .map(|s| s.collect::<Vec<&str>>())
                .map(|s| {
                    s.iter()
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|f| Board {
            grid: f,
            state: Grid::new(5, 5),
        })
        .collect::<Vec<_>>();

    let mut winner_call = -1;
    let mut winner: Option<&Board> = None;
    for call in call_order {
        boards.iter_mut().for_each(|b| b.call(call));
        winner = boards.iter().filter(|b| b.is_winner()).next();
        if winner.is_some() {
            winner_call = call;
            break;
        }
    }
    println!("Winner call: {}", winner_call);
    println!("Board: {:?}", winner.unwrap().state);
    let score = winner.unwrap().score();
    println!("Score: {}", score);

    let result = score * winner_call;
    println!("Part 1 Result: {}", result);
    return result;
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    let input = data.split("\n").collect::<Vec<&str>>();
    let call_order = input[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
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
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|f| Board {
            grid: f,
            state: Grid::new(5, 5),
        })
        .collect::<Vec<_>>();

    let mut winner_call = -1;
    let mut winner: Option<&mut Board> = None;
    let mut remaining = boards;
    for call in call_order {
        remaining.iter_mut().for_each(|b| b.call(call));
        let r = remaining
            .drain_filter(|b| !b.is_winner())
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
