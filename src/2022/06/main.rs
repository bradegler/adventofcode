use aocshared::aoc::aoc::*;
use array_tool::vec::Uniq;

const YEAR: i32 = 2022;
const DAY: u32 = 06;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let content: Vec<char> = data.chars().collect();
    let mut window = 3;
    let mut windows = content.windows(4);
    loop {
        window += 1;
        match windows.next() {
            Some(win) => {
                if win.to_vec().is_unique() {
                    break;
                }
            }
            None => break,
        }
    }
    window
}

fn part2(data: &String) -> u64 {
    let content: Vec<char> = data.chars().collect();
    let mut window = 13;
    let mut windows = content.windows(14);
    loop {
        window += 1;
        match windows.next() {
            Some(win) => {
                if win.to_vec().is_unique() {
                    break;
                }
            }
            None => break,
        }
    }
    window
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2022_06_ep1() {
        assert_eq!(7, part1(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
        assert_eq!(5, part1(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")));
        assert_eq!(6, part1(&String::from("nppdvjthqldpwncqszvftbrmjlhg")));
        assert_eq!(
            10,
            part1(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))
        );
        assert_eq!(11, part1(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
    }

    #[test]
    fn t2022_06_ep2() {
        assert_eq!(19, part2(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
        assert_eq!(23, part2(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")));
        assert_eq!(23, part2(&String::from("nppdvjthqldpwncqszvftbrmjlhg")));
        assert_eq!(
            29,
            part2(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))
        );
        assert_eq!(26, part2(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
    }

    #[test]
    fn t2022_06_rp1() {
        assert_eq!(1794, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_06_rp2() {
        assert_eq!(2851, part2(&get_input(YEAR, DAY)));
    }
}
