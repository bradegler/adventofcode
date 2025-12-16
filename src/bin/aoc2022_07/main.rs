use aocshared::aoc::aoc::*;

const YEAR: i32 = 2022;
const DAY: u32 = 07;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn build_directories(lines: Vec<&str>) -> Vec<u32> {
    let mut tmp_dir: Vec<u32> = Vec::new();
    let mut directories: Vec<u32> = Vec::new();

    for line in lines {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => directories.push(tmp_dir.pop().unwrap()),
            ["$", "cd", _] => tmp_dir.push(0),
            [size, _] => {
                if let Ok(num) = size.parse::<u32>() {
                    tmp_dir.iter_mut().for_each(|n| *n += num)
                }
            }
            [..] => unreachable!(),
        }
    }
    directories.extend(tmp_dir);
    directories
}

fn part1(data: &String) -> u32 {
    let lines = get_lines_as_strs(data);
    let directories = build_directories(lines);
    directories
        .iter()
        .filter(|&&size| size < 100000)
        .sum::<u32>()
}

fn part2(data: &String) -> u32 {
    let lines = get_lines_as_strs(data);
    let directories = build_directories(lines);
    let root = *directories.iter().max().unwrap();
    let required = root + 30000000 - 70000000;
    *directories
        .iter()
        .filter(|&&dir| dir >= required)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2022_07_ep1() {
        assert_eq!(95437, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_07_ep2() {
        assert_eq!(24933642, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_07_rp1() {
        assert_eq!(1086293, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_07_rp2() {
        assert_eq!(366028, part2(&get_input(YEAR, DAY)));
    }
}
