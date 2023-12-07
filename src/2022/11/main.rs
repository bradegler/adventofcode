use aocshared::aoc::aoc::*;
use std::fmt::Debug;
use std::str::FromStr;
use std::str::Lines;

const YEAR: i32 = 2022;
const DAY: u32 = 11;

type Operation = Box<dyn Fn(i64) -> i64>;

struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    on_true: usize,
    on_false: usize,
    inspections: usize,
}

// Helpers for parsing:

fn strip_next<'a>(lines: &mut Lines<'a>, prefix: &str) -> &'a str {
    lines.next().unwrap().strip_prefix(prefix).unwrap()
}

fn parse_next<F>(lines: &mut Lines, prefix: &str) -> F
where
    F: FromStr,
    <F as FromStr>::Err: Debug,
{
    strip_next(lines, prefix).parse().unwrap()
}

fn parse_items(lines: &mut Lines, prefix: &str) -> Vec<i64> {
    strip_next(lines, prefix)
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn parse_operation(lines: &mut Lines, prefix: &str) -> Operation {
    let operands: Vec<_> = strip_next(lines, prefix).split(' ').collect();

    match operands[..] {
        ["old", "*", "old"] => Box::new(|x| x * x),

        ["old", "*", y] => {
            let y: i64 = y.parse().unwrap();
            Box::new(move |x| x * y)
        }

        ["old", "+", y] => {
            let y: i64 = y.parse().unwrap();
            Box::new(move |x| x + y)
        }

        _ => panic!("{operands:?}"),
    }
}

fn parse(lines: &mut Lines) -> Vec<Monkey> {
    let mut result = Vec::new();
    loop {
        lines.next(); // Skip monkey ID.

        result.push(Monkey {
            items: parse_items(lines, "  Starting items: "),
            operation: parse_operation(lines, "  Operation: new = "),
            test: parse_next(lines, "  Test: divisible by "),
            on_true: parse_next(lines, "    If true: throw to monkey "),
            on_false: parse_next(lines, "    If false: throw to monkey "),
            inspections: 0,
        });

        if lines.next().is_none() {
            break;
        }
    }

    result
}

fn solve<R>(mut monkeys: Vec<Monkey>, rounds: usize, reduction: R) -> usize
where
    R: Fn(i64) -> i64,
{
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            // Apply math to items.
            let items: Vec<_> = monkey
                .items
                .drain(..)
                .map(|item| (monkey.operation)(item))
                .map(&reduction)
                .collect();

            // Count inspections.
            monkey.inspections += items.len();

            // Throw items to other monkyes.
            let test = monkey.test;
            let on_true = monkey.on_true;
            let on_false = monkey.on_false;
            for item in items {
                let target = if item % test == 0 { on_true } else { on_false };
                monkeys[target].items.push(item);
            }
        }
    }

    // Extract inspection counts.
    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    // Return product of top two inspection counts.
    inspections.sort_unstable_by(|a, b| b.cmp(a));
    inspections.iter().take(2).product()
}

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> usize {
    let monkeys = parse(&mut data.lines());
    solve(monkeys, 20, |x| x / 3)
}

fn part2(data: &String) -> usize {
    let monkeys = parse(&mut data.lines());
    let lcd: i64 = monkeys.iter().map(|m| m.test).product();
    solve(monkeys, 10_000, |x| x % lcd)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2022_11_ep1() {
        assert_eq!(10605, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_11_ep2() {
        assert_eq!(2713310158, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_11_rp1() {
        assert_eq!(72884, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_11_rp2() {
        assert_eq!(15310845153, part2(&get_input(YEAR, DAY)));
    }
}
