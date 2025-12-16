use aocshared::aoc::aoc::*;

const YEAR: i32 = 2022;
const DAY: u32 = 10;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

const CMD_NOOP: &str = "noop";
//const CMD_ADDX: &str = "addx";
const LIGHT: char = '#';
const DARK: char = '.';

fn part1(data: &String) -> i32 {
    let interesting_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let instrs = get_lines_as_strs(data);
    let mut x_reg = 1;
    let mut cycle = 1;
    let mut result = 0;
    for inst in instrs {
        if inst == CMD_NOOP {
            if interesting_cycles.contains(&cycle) {
                result = result + (cycle * x_reg)
            }
            cycle += 1;
        } else {
            if interesting_cycles.contains(&cycle) {
                result = result + (cycle * x_reg)
            }
            cycle += 1;
            let amt = inst
                .split(" ")
                .last()
                .map(|s| s.parse::<i32>().unwrap())
                .unwrap();
            if interesting_cycles.contains(&cycle) {
                result = result + (cycle * x_reg)
            }
            cycle += 1;
            x_reg += amt;
        }
    }

    result
}

fn cycle(x_reg: i32, position: i32) -> i32 {
    let pixel = if vec![x_reg - 1, x_reg, x_reg + 1].contains(&(position - (40 * (position / 40))))
    {
        LIGHT
    } else {
        DARK
    };
    print!("{}", pixel);
    if (position + 1) % 40 == 0 {
        print!("\n")
    }
    position + 1
}
fn part2(data: &String) -> u64 {
    let instrs = get_lines_as_strs(data);
    let mut x_reg: i32 = 1;
    let mut position: i32 = 0;
    for inst in instrs {
        if inst == CMD_NOOP {
            position = cycle(x_reg, position);
        } else {
            position = cycle(x_reg, position);
            position = cycle(x_reg, position);
            x_reg += inst
                .split(" ")
                .last()
                .map(|s| s.parse::<i32>().unwrap())
                .unwrap();
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2022_10_ep1() {
        assert_eq!(13140, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_10_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_10_rp1() {
        assert_eq!(14240, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_10_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
