use aocshared::aoc::aoc::*;

const YEAR: i32 = 2020;
const DAY: u32 = 05;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    get_lines_as_strs(data)
        .iter()
        .map(|s| calc_seat_number(s))
        .max()
        .unwrap()
}

fn part2(data: &String) -> i32 {
    let mut passes = get_lines_as_strs(data)
        .iter()
        .map(|s| calc_seat_number(s))
        .collect::<Vec<i32>>();
    println!("{}", passes.len());
    passes.sort();
    passes.windows(2).find(|w| w[1] - w[0] == 2).unwrap()[0] + 1
}

fn calc_seat_number(inst: &str) -> i32 {
    let rs: i16 = 0;
    let re: i16 = 127;
    let row = inst.chars().collect::<Vec<char>>()[0..7]
        .iter()
        .fold(rs..=re, |acc, c| {
            // println!("c = {}: acc = {}..{}", c, acc.start(), acc.end());
            if acc.len() == 3 {
                let mid = acc.start() + 1;
                return mid..=mid;
            }
            match c {
                'F' => *acc.start()..=(acc.start() + ((acc.end() - acc.start()) / 2)),
                'B' => (acc.start() + ((acc.end() + 1 - acc.start()) / 2))..=*acc.end(),
                _ => panic!("Invalid row {}", c),
            }
        });
    // println!("Row: {}..{}", row.start(), row.end());
    let cs: i16 = 0;
    let ce: i16 = 7;
    let col = inst.chars().collect::<Vec<char>>()[7..]
        .iter()
        .fold(cs..=ce, |acc, c| {
            // println!("c = {}: acc = {}..{}", c, acc.start(), acc.end());
            if acc.len() == 3 {
                let mid = acc.start() + 1;
                return mid..=mid;
            }
            match c {
                'L' => *acc.start()..=(acc.start() + ((acc.end() - acc.start()) / 2)),
                'R' => (acc.start() + ((acc.end() + 1 - acc.start()) / 2))..=*acc.end(),
                _ => panic!("Invalid col {}", c),
            }
        });
    // println!("Col: {}..{}", col.start(), col.end());
    let seat = row.start() * 8 + col.start();
    // println!("Seat: {}", seat);
    seat.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2020_05_calc_seat() {
        assert_eq!(357, calc_seat_number("FBFBBFFRLR"));
        assert_eq!(567, calc_seat_number("BFFFBBFRRR"));
        assert_eq!(119, calc_seat_number("FFFBBBFRRR"));
        assert_eq!(820, calc_seat_number("BBFFBBFRLL"));
    }

    #[test]
    fn t2020_05_rp1() {
        assert_eq!(880, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_05_rp2() {
        assert_eq!(731, part2(&get_input(YEAR, DAY)));
    }
}
