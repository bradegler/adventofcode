use aocshared::*;
use std::collections::HashSet;

const YEAR: i32 = 2021;
const DAY: u32 = 15;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn get_adj_points(
    (y, x): (usize, usize),
    (width, height): (usize, usize),
    allow_diagonal: bool,
) -> Vec<(usize, usize)> {
    let adj: Vec<(i32, i32)> = if allow_diagonal {
        vec![
            (y as i32 - 1, x as i32),
            (y as i32 + 1, x as i32),
            (y as i32, x as i32 - 1),
            (y as i32, x as i32 + 1),
            (y as i32 - 1, x as i32 - 1),
            (y as i32 + 1, x as i32 + 1),
            (y as i32 - 1, x as i32 + 1),
            (y as i32 + 1, x as i32 - 1),
        ]
    } else {
        vec![
            (y as i32 - 1, x as i32),
            (y as i32 + 1, x as i32),
            (y as i32, x as i32 - 1),
            (y as i32, x as i32 + 1),
        ]
    };
    adj.into_iter()
        .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < height as i32 && *x < width as i32)
        .map(|(y, x)| (y as usize, x as usize))
        .collect::<Vec<(usize, usize)>>()
}

fn solve(grid: &Vec<Vec<usize>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();

    let mut stack = [((0, 0), 0)].to_vec();
    while !stack.is_empty() {
        let (pos, cost) = stack.pop().unwrap();
        if pos == (cols - 1, rows - 1) {
            return cost;
        }

        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        for (x1, y1) in get_adj_points(pos, (cols, rows), false) {
            stack.push(((x1, y1), cost + grid[y1][x1]));
        }

        stack.sort_by(|(_, cost_a), (_, cost_b)| return cost_b.cmp(&cost_a));
    }
    return 0;
}
fn part1(data: &String) -> usize {
    let input = get_lines_as_vec_usize(data);
    solve(&input)
}

fn expand_grid(grid_orig: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut grid = grid_orig.clone();
    let cols_orig = grid[0].len();
    let rows_orig = grid.len();
    grid.resize(rows_orig * 5, Vec::new());
    grid.iter_mut().for_each(|col| col.resize(cols_orig * 5, 0));
    let cols = grid[0].len();
    let rows = grid.len();

    for row in 0..rows {
        for col in 0..cols {
            let q_row = row / rows_orig;
            let q_col = col / cols_orig;
            let value = grid_orig[row % rows_orig][col % cols_orig] + q_row + q_col;
            grid[row][col] = value % 10 + value / 10;
        }
    }
    grid
}

fn part2(data: &String) -> usize {
    let input = get_lines_as_vec_usize(data);
    let grid = expand_grid(&input);
    solve(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_15_ep1() {
        assert_eq!(40, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_15_ep2() {
        assert_eq!(315, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_15_rp1() {
        assert_eq!(523, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_15_rp2() {
        assert_eq!(2876, part2(&get_input(YEAR, DAY)));
    }
}
