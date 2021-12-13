use aocshared::*;
use regex::Regex;
use std::collections::HashSet;

const YEAR: i32 = 2020;
const DAY: u32 = 08;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

#[derive(Clone)]
struct Code {
    instructions: Vec<(String, i32)>,
    execution_pointer: i32,
    accumulator: i64,
    running: bool,
    infinite_loop: bool,
    executed: HashSet<i32>,
}

impl Code {
    fn new(ins: Vec<(String, i32)>) -> Self {
        Code {
            instructions: ins,
            execution_pointer: 0,
            accumulator: 0i64,
            running: false,
            infinite_loop: false,
            executed: HashSet::new(),
        }
    }
    fn run(&mut self) -> bool {
        self.running = true;
        while self.running {
            let instruction = &self.instructions[self.execution_pointer as usize];
            match instruction.0.as_str() {
                "nop" => self.execution_pointer += 1,
                "acc" => {
                    self.accumulator += instruction.1 as i64;
                    self.execution_pointer += 1
                }
                "jmp" => self.execution_pointer += instruction.1,
                _ => panic!("Unknown instruction"),
            }
            if self.executed.contains(&self.execution_pointer) {
                self.infinite_loop = true;
            }
            if self.execution_pointer >= self.instructions.len() as i32 {
                self.running = false;
            }
            if self.infinite_loop {
                self.running = false;
            }
            self.executed.insert(self.execution_pointer);
        }
        !self.infinite_loop
    }
}

fn part1(data: &String) -> i64 {
    let lines = get_lines_as_strs(data);
    let pattern = Regex::new(r"(\w{3}) (\+\d+|-\d+)").unwrap();
    let ins = lines
        .iter()
        .map(|line| {
            let caps = pattern.captures(line).unwrap();
            let p1 = caps[1].to_string();
            let p2 = caps[2].replace("+", "").parse::<i32>().unwrap();
            (p1, p2)
        })
        .collect::<Vec<(String, i32)>>();
    let mut code = Code::new(ins.clone());
    code.run();
    code.accumulator
}

fn part2(data: &String) -> i64 {
    let lines = get_lines_as_strs(data);
    let pattern = Regex::new(r"(\w{3}) (\+\d+|-\d+)").unwrap();
    let ins = lines
        .iter()
        .map(|line| {
            let caps = pattern.captures(line).unwrap();
            let p1 = caps[1].to_string();
            let p2 = caps[2].replace("+", "").parse::<i32>().unwrap();
            (p1, p2)
        })
        .collect::<Vec<(String, i32)>>();

    for idx in 0..ins.len() {
        let mut should_run = false;
        let mut new_ins = ins.clone();
        if new_ins[idx].0 == "nop" {
            new_ins[idx].0 = "jmp".to_string();
            should_run = true;
        } else if new_ins[idx].0 == "jmp" {
            new_ins[idx].0 = "nop".to_string();
            should_run = true;
        }
        if should_run {
            let mut code = Code::new(new_ins);
            code.run();
            if !code.infinite_loop {
                return code.accumulator;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2020_08_ep1() {
        assert_eq!(5, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_08_ep2() {
        assert_eq!(8, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_08_rp1() {
        assert_eq!(1548, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_08_rp2() {
        assert_eq!(1375, part2(&get_input(YEAR, DAY)));
    }
}
