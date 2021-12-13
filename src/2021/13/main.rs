use aocshared::*;
use regex::Regex;

const YEAR: i32 = 2021;
const DAY: u32 = 13;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> usize {
    let lines = get_lines_as_strs(data);
    let mut grid = vec![vec!['.'; 2000]; 2000];
    let mut ins = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("fold along") {
            // Capture the fold instruction
            ins.push(line);
        } else {
            // Parse the line
            let mut v = line.split(',');
            let x = v.next().unwrap().trim().parse::<usize>().unwrap();
            let y = v.next().unwrap().trim().parse::<usize>().unwrap();
            grid[y as usize][x as usize] = '#';
        }
    }
    let pattern = Regex::new(r"^fold along (\w)=(\d+)$").unwrap();
    let fold = ins[0];
    let caps = pattern.captures(fold).unwrap();
    let axis = caps.get(1).unwrap().as_str();
    let point = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    // println!("Fold along {}={}", axis, value);
    if axis == "x" {
        (0..2000).for_each(|i| grid[i][point] = '|');
        let mut a = point - 1;
        let mut b = point + 1;
        loop {
            (0..2000).for_each(|i| {
                if grid[i][b] == '#' {
                    grid[i][a] = '#';
                }
                grid[i][b] = '.';
            });
            if a == 0 || b == 1999 {
                break;
            }
            a -= 1;
            b += 1;
        }
    } else {
        (0..2000).for_each(|j| grid[point][j] = '-');
        let mut a = point - 1;
        let mut b = point + 1;
        loop {
            (0..2000).for_each(|j| {
                if grid[b][j] == '#' {
                    grid[a][j] = '#';
                }
                grid[b][j] = '.';
            });
            if a == 0 || b == 1999 {
                break;
            }
            a -= 1;
            b += 1;
        }
    }
    for i in 0..50 {
        for j in 0..50 {
            print!("{} ", grid[i][j]);
        }
        println!();
    }
    grid.iter()
        .map(|row| row.iter().map(|p| (*p == '#') as usize).sum::<usize>())
        .sum::<usize>()
}

fn part2(data: &String) -> usize {
    let lines = get_lines_as_strs(data);
    let mut grid = vec![vec!['.'; 2000]; 2000];
    let mut ins = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("fold along") {
            // Capture the fold instruction
            ins.push(line);
        } else {
            // Parse the line
            let mut v = line.split(',');
            let x = v.next().unwrap().trim().parse::<usize>().unwrap();
            let y = v.next().unwrap().trim().parse::<usize>().unwrap();
            grid[y as usize][x as usize] = '#';
        }
    }
    let pattern = Regex::new(r"^fold along (\w)=(\d+)$").unwrap();
    for fold in ins {
        let caps = pattern.captures(fold).unwrap();
        let axis = caps.get(1).unwrap().as_str();
        let point = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        // println!("Fold along {}={}", axis, value);
        if axis == "x" {
            (0..2000).for_each(|i| grid[i][point] = '|');
            if point == 0 || point == 1999 {
                continue;
            }
            let mut a = point - 1;
            let mut b = point + 1;
            loop {
                (0..2000).for_each(|i| {
                    if grid[i][b] == '#' {
                        grid[i][a] = '#';
                    }
                    grid[i][b] = '.';
                });
                if a == 0 || b == 1999 {
                    break;
                }
                a -= 1;
                b += 1;
            }
        } else {
            (0..2000).for_each(|j| grid[point][j] = '-');
            if point == 0 || point == 1999 {
                continue;
            }
            let mut a = point - 1;
            let mut b = point + 1;
            loop {
                (0..2000).for_each(|j| {
                    if grid[b][j] == '#' {
                        grid[a][j] = '#';
                    }
                    grid[b][j] = '.';
                });
                if a == 0 || b == 1999 {
                    break;
                }
                a -= 1;
                b += 1;
            }
        }
    }
    for i in 0..50 {
        for j in 0..50 {
            print!("{} ", grid[i][j]);
        }
        println!();
    }
    grid.iter()
        .map(|row| row.iter().map(|p| (*p == '#') as usize).sum::<usize>())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_13_ep1() {
        assert_eq!(17, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_13_ep2() {
        assert_eq!(16, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_13_rp1() {
        assert_eq!(818, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_13_rp2() {
        // LRGPRECB
        assert_eq!(101, part2(&get_input(YEAR, DAY)));
    }
}
