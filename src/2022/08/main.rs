use std::cmp::max;

use aocshared::*;

const YEAR: i32 = 2022;
const DAY: u32 = 08;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let grid = get_lines_as_grid_u32(data);
    let max_x = grid.len();
    let max_y = grid[0].len();

    let mut count = 0;
    for row in 0..max_y {
        for col in 0..max_x {
            if is_visible(row, col, &grid) {
                count += 1;
            }
        }
    }
    count
}

fn is_visible(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> bool {
    if col == 0 || row == 0 || col == grid[0].len() - 1 || row == grid.len() - 1 {
        return true;
    }

    let tree = grid[row][col];
    let visible_top = !(0..row).rev().any(|r| grid[r][col] >= tree);
    let visible_bottom = !(row + 1..grid.len()).any(|r| grid[r][col] >= tree);
    let visible_left = !grid[row][0..col].iter().rev().any(|num| num >= &tree);
    let visible_right = !grid[row][col + 1..].iter().any(|num| num >= &tree);
    visible_top || visible_bottom || visible_left || visible_right
}

fn part2(data: &String) -> u64 {
    let grid = get_lines_as_grid_u32(data);
    let max_x = grid.len();
    let max_y = grid[0].len();

    let mut max_score = 0;
    for row in 0..max_y {
        for col in 0..max_x {
            let s = score(row, col, &grid);
            if s > max_score {
                max_score = s;
            }
        }
    }
    max_score as u64
}

fn score(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> usize {
    if col == 0 || row == 0 || col == grid[0].len() - 1 || row == grid.len() - 1 {
        return 0;
    }

    let tree = grid[row][col];
    let mut scores: Vec<usize> = vec![0, 0, 0, 0];

    for r in (0..row).rev() {
        scores[0] += 1;
        if grid[r][col] >= tree {
            break;
        }
    }
    for r in row + 1..grid.len() {
        scores[1] += 1;
        if grid[r][col] >= tree {
            break;
        }
    }

    for num in grid[row][0..col].iter().rev() {
        scores[2] += 1;
        if num >= &tree {
            break;
        }
    }
    for num in &grid[row][col + 1..] {
        scores[3] += 1;
        if num >= &tree {
            break;
        }
    }
    println!("score 1:[{:?},{:?}] {:?}", row, col, scores);
    scores.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2022_08_ep1() {
        assert_eq!(21, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_08_ep2() {
        assert_eq!(8, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_08_rp1() {
        assert_eq!(1695, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_08_rp2() {
        assert_eq!(287040, part2(&get_input(YEAR, DAY)));
    }
}
