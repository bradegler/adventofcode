use aocshared::aoc::aoc::*;

const YEAR: i32 = 2021;
const DAY: u32 = 25;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

const EAST_MOVER: char = '>';
const SOUTH_MOVER: char = 'v';
const EMPTY: char = '.';

fn part1(data: &String) -> i32 {
    let mut grid = get_lines_as_vec_chars(&data);
    let mut moved = true;
    let mut step = 0;
    while moved {
        moved = false;
        step += 1;
        let mut new_grid = grid.clone();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let cucumber = grid[y][x];
                if cucumber == EAST_MOVER {
                    let target_x = if x == grid[y].len() - 1 { 0 } else { x + 1 };
                    let target = grid[y][target_x];
                    if target == EMPTY {
                        new_grid[y][target_x] = EAST_MOVER;
                        new_grid[y][x] = EMPTY;
                        moved = true;
                    }
                }
            }
        }
        grid = new_grid.clone();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let cucumber = grid[y][x];
                if cucumber == SOUTH_MOVER {
                    let target_y = if y == grid.len() - 1 { 0 } else { y + 1 };
                    let target = grid[target_y][x];
                    if target == EMPTY {
                        new_grid[target_y][x] = SOUTH_MOVER;
                        new_grid[y][x] = EMPTY;
                        moved = true;
                    }
                }
            }
        }
        grid = new_grid;
    }
    step
}

fn part2(_data: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2021_25_ep1() {
        assert_eq!(58, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_25_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_25_rp1() {
        assert_eq!(424, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_25_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
