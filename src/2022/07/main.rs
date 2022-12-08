use aocshared::*;
use hashbrown::HashMap;
use itertools::Itertools;
use regex::{Match, Regex};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

const YEAR: i32 = 2022;
const DAY: u32 = 07;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn build_tree(lines: Vec<&str>) -> HashMap<String, Vec<(u64, &str)>> {
    let re = Regex::new(r"^\$ cd (.*)$").unwrap();
    let de = Regex::new(r"^(.*)(/.*)$").unwrap();
    let mut nodes = HashMap::<String, Vec<(u64, &str)>>::new();
    let root = "";
    nodes.insert(root.to_string(), vec![]);
    let mut cur_node = root;
    let mut nn = "".to_string();
    for line in lines {
        match line {
            l if l.starts_with("dir") => {
                let name = line.split(" ").last().unwrap();
                let dir_name = format!("{cur_node}/{name}");
                nodes.insert(dir_name, vec![]);
            }
            l if line.starts_with("$ ls") => (),
            l if line.starts_with("$ cd") => {
                let dir = re.captures(l).unwrap().get(1).unwrap().as_str();
                match dir {
                    "/" => cur_node = &root,
                    ".." => {
                        let c = de.captures(cur_node).unwrap().get(1);
                        cur_node = match c {
                            Some(s) => s.as_str(),
                            None => "",
                        };
                    }
                    x => {
                        nn = format!("{cur_node}/{x}");
                        cur_node = nn.as_str();
                    }
                }
            }
            x => {
                let parts = x.split(" ").collect_vec();
                println!("{} - {:?}", x, parts);
                let size = parts[0].parse::<u64>().unwrap();
                let name = parts[1];
                nodes.get_mut(cur_node).unwrap().push((size, name));
            }
        }
    }
    nodes
}

fn part1(data: &String) -> u32 {
    let lines = get_lines_as_strs(data);
    let mut tmp_dir: Vec<u32> = Vec::new();
    let mut directories: Vec<u32> = Vec::new();

    for line in lines {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => directories.push(tmp_dir.pop().unwrap()),
            ["$", "cd", _] => tmp_dir.push(0),
            [size, _] => {
                // Getting rid of "dir ..." and "$ ls" here
                if let Ok(num) = size.parse::<u32>() {
                    tmp_dir.iter_mut().for_each(|n| *n += num)
                }
            }
            [..] => unreachable!(),
        }
    }
    directories.extend(tmp_dir);

    directories
        .iter()
        .filter(|&&size| size < 100_000)
        .sum::<u32>()
}

fn part2(data: &String) -> u32 {
    let lines = get_lines_as_strs(data);
    let mut tmp_dir: Vec<u32> = Vec::new();
    let mut directories: Vec<u32> = Vec::new();

    for line in lines {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => directories.push(tmp_dir.pop().unwrap()),
            ["$", "cd", _] => tmp_dir.push(0),
            [size, _] => {
                // Getting rid of "dir ..." and "$ ls" here
                if let Ok(num) = size.parse::<u32>() {
                    tmp_dir.iter_mut().for_each(|n| *n += num)
                }
            }
            [..] => unreachable!(),
        }
    }
    directories.extend(tmp_dir);
    let root = *directories.iter().max().unwrap(); // Biggest directory
    let required = root + 30_000_000 - 70_000_000; // Required size for dir
    *directories
        .iter()
        .filter(|&&dir| dir >= required)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
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
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
