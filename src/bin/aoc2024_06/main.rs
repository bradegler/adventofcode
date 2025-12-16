use aocshared::{
    aoc::aoc::*,
    grid::grid::{Grid, Visitable},
};
use hashbrown::HashSet;
use itertools::Itertools;

const YEAR: i32 = 2024;
const DAY: u32 = 06;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let contents = get_lines_as_grid_char(data);
    let grid = Grid::new(contents);
    let path = guard_movement(&grid);
    path.unwrap().iter().count() as u64
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn guard_movement(gr: &Grid<char>) -> Option<Vec<(usize, usize)>> {
    let mut grid = gr.clone();
    let start = grid
        .into_iter()
        .find(|(_, _, c)| *c == '^')
        .map(|(y, x, _)| (y, x))
        .unwrap();
    // there should always be only one start
    let mut prev = start;
    grid.visit((prev.0, prev.1));
    let mut next = Some((prev.0 - 1, prev.1));
    while next.is_some() {
        let mut n = next.unwrap();
        if !grid.in_bounds((n.0 as i32, n.1 as i32)) {
            break;
        }
        if grid.at(n) == '#' {
            n = match (prev, n) {
                // up -> right
                ((py, px), (ny, nx)) if py > ny && px == nx => (py, px + 1),
                // down -> left
                ((py, px), (ny, nx)) if py < ny && px == nx => (py, px - 1),
                // left -> up
                ((py, px), (ny, nx)) if py == ny && px > nx => (py - 1, px),
                // right -> down
                ((py, px), (ny, nx)) if py == ny && px < nx => (py + 1, px),
                _ => panic!("Not moving"),
            };
        }
        grid.visit(n);
        next = grid.get_next_in_seq(prev, n);
        prev = n;
    }
    Some(
        grid.into_iter()
            .filter(|(y, x, _)| grid.is_visited((*y, *x)))
            .map(|(y, x, _)| (y, x))
            .collect_vec(),
    )
}

pub fn part2(data: &String) -> u32 {
    let mut guard = None;
    let mut grid = get_lines_as_grid_char(data);
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                guard = Some(Guard {
                    direction: Direction::Up,
                    position: Point::new(x as i32, y as i32),
                });
                break;
            }
        }
    }
    let mut guard = guard.unwrap();
    grid[guard.position.y as usize][guard.position.x as usize] = '.';

    let mut positions = HashSet::with_capacity(10_000);
    let mut obstacles = 0u32;

    loop {
        let next_pt = Point::from(guard.direction);
        let next = Point::new(guard.position.x + next_pt.x, guard.position.y + next_pt.y);

        if let Some(row) = grid.get(next.y as usize) {
            if let Some(position) = row.get(next.x as usize) {
                match position {
                    '.' => {
                        if !positions.contains(&next) {
                            grid[next.y as usize][next.x as usize] = '#';

                            if is_loop(&grid, guard) {
                                obstacles += 1;
                            }

                            grid[next.y as usize][next.x as usize] = '.';
                        }

                        guard.position = next;
                        positions.insert(next);
                    }
                    '#' => guard.direction = turn(guard.direction),
                    _ => unreachable!(),
                }

                continue;
            }
        }

        break;
    }

    obstacles
}

fn turn(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn is_loop(grid: &[Vec<char>], mut guard: Guard) -> bool {
    let mut turns = HashSet::with_capacity(500);

    loop {
        let next_pt = Point::from(guard.direction);
        let next = Point::new(guard.position.x + next_pt.x, guard.position.y + next_pt.y);

        if let Some(row) = grid.get(next.y as usize) {
            if let Some(position) = row.get(next.x as usize) {
                match position {
                    '.' => guard.position = next,
                    '#' => {
                        guard.direction = turn(guard.direction);

                        if turns.contains(&guard) {
                            return true;
                        }

                        turns.insert(guard);
                    }
                    _ => unreachable!(),
                }

                continue;
            }
        }

        break;
    }

    false
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Guard {
    direction: Direction,
    position: Point,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2024_06_ep1() {
        assert_eq!(41, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_06_ep2() {
        assert_eq!(6, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_06_rp1() {
        assert_eq!(5312, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_06_rp2() {
        assert_eq!(1748, part2(&get_input(YEAR, DAY)));
    }
}
