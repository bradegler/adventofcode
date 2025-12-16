use std::fmt;

use aocshared::aoc::aoc::*;
use aocshared::grid::grid::{Grid, Point, Visitable};
use itertools::Itertools;

const YEAR: i32 = 2023;
const DAY: u32 = 10;

const NORTH_SOUTH: char = '|';
const EAST_WEST: char = '-';
const NORTH_EAST: char = 'L';
const NORTH_WEST: char = 'J';
const SOUTH_WEST: char = '7';
const SOUTH_EAST: char = 'F';
const GROUND: char = '.';
const START: char = 'S';

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn map_tile(tile: char) -> Direction {
    match tile {
        NORTH_SOUTH => Direction::NS,
        EAST_WEST => Direction::EW,
        NORTH_EAST => Direction::NE,
        NORTH_WEST => Direction::NW,
        SOUTH_WEST => Direction::SW,
        SOUTH_EAST => Direction::SE,
        GROUND => Direction::Ground,
        START => Direction::Start,
        _ => panic!("Unmapped char! {}", tile),
    }
}

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn can_move(
    grid: &Grid<char>,
    (curr_y, curr_x): Point<usize>,
    (next_y, next_x): Point<usize>,
) -> Option<(usize, usize)> {
    let some = Some((next_y, next_x));
    let curr_char = grid.at((curr_y, curr_x));
    let next_char = grid.at((next_y, next_x));

    let same_x = next_x == curr_x;
    let same_y = next_y == curr_y;

    let north = next_y < curr_y && same_x;
    let south = next_y > curr_y && same_x;
    let west = next_x < curr_x && same_y;
    let east = next_x > curr_x && same_y;

    let res = match (curr_char, next_char) {
        (GROUND, _) => None,
        (_, START) => None,
        (_, GROUND) => None,
        (START, NORTH_SOUTH) if !same_y && same_x => some,
        (START, EAST_WEST) if same_y && !same_x => some,

        (START, NORTH_EAST) if south || west => some,
        (START, NORTH_WEST) if south || east => some,
        (START, SOUTH_EAST) if north || west => some,
        (START, SOUTH_WEST) if north || east => some,

        (NORTH_SOUTH, NORTH_SOUTH) if !same_y && same_x => some,
        (NORTH_SOUTH, SOUTH_EAST) if north => some,
        (NORTH_SOUTH, SOUTH_WEST) if north => some,
        (NORTH_SOUTH, NORTH_EAST) if south => some,
        (NORTH_SOUTH, NORTH_WEST) if south => some,

        (EAST_WEST, EAST_WEST) if same_y && !same_x => some,
        (EAST_WEST, SOUTH_EAST) if west => some,
        (EAST_WEST, SOUTH_WEST) if east => some,
        (EAST_WEST, NORTH_EAST) if west => some,
        (EAST_WEST, NORTH_WEST) if east => some,

        (SOUTH_WEST, NORTH_SOUTH) if south => some,
        (SOUTH_WEST, EAST_WEST) if west => some,
        (SOUTH_WEST, SOUTH_EAST) if west => some,
        (SOUTH_WEST, NORTH_EAST) if west || south => some,
        (SOUTH_WEST, NORTH_WEST) if south => some,

        (NORTH_WEST, NORTH_SOUTH) if north => some,
        (NORTH_WEST, EAST_WEST) if west => some,
        (NORTH_WEST, NORTH_EAST) if west => some,
        (NORTH_WEST, SOUTH_EAST) if west || north => some,
        (NORTH_WEST, SOUTH_WEST) if north => some,

        (SOUTH_EAST, NORTH_SOUTH) if south => some,
        (SOUTH_EAST, EAST_WEST) if east => some,
        (SOUTH_EAST, SOUTH_WEST) if east => some,
        (SOUTH_EAST, NORTH_WEST) if east || south => some,
        (SOUTH_EAST, NORTH_EAST) if south => some,

        (NORTH_EAST, NORTH_SOUTH) if north => some,
        (NORTH_EAST, EAST_WEST) if east => some,
        (NORTH_EAST, NORTH_WEST) if east => some,
        (NORTH_EAST, SOUTH_WEST) if east || north => some,
        (NORTH_EAST, SOUTH_EAST) if north => some,
        _ => None,
    };
    res
}

fn valid_moves(grid: &Grid<char>, (curr_y, curr_x): Point<usize>) -> Vec<(usize, usize)> {
    let points = grid.get_adj_points((curr_y, curr_x), false);
    let mut search = vec![];
    for (adj_y, adj_x) in points {
        match can_move(grid, (curr_y, curr_x), (adj_y, adj_x)) {
            Some(p) => {
                search.push(p);
            }
            _ => {}
        }
    }
    return search;
}

fn part1(data: &String) -> u64 {
    let contents = get_lines_as_grid_char(data);
    let mut grid = Grid::new(contents);
    let spoint = grid.find(START)[0];
    let mut search = valid_moves(&grid, spoint);
    let mut distance = vec![vec![0; grid.width()]; grid.height()];
    let mut current_distance = 0;
    while search.len() > 0 {
        let mut next = vec![];
        current_distance += 1;
        for (y, x) in search.into_iter() {
            let visited = grid.is_visited((y, x));
            if !visited {
                grid.visit((y, x));
                distance[y][x] = current_distance;
                valid_moves(&grid, (y, x))
                    .iter()
                    .for_each(|m| next.push(*m));
            }
        }
        search = next.clone();
    }
    print_grid(&distance);
    *distance
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap() as u64
}

fn part2(data: &String) -> u64 {
    let contents = get_lines_as_grid_char(data)
        .iter()
        .map(|r| r.into_iter().map(|c| map_tile(*c)).collect_vec())
        .collect_vec();
    let grid = Grid::new(contents);
    let spoint = grid.find(Direction::Start)[0];
    let start_coord = Coord {
        y: spoint.0,
        x: spoint.1,
    };
    let start_type = get_start_type(start_coord, &grid);
    let mut loop_path: Vec<Coord> = vec![start_coord];
    let (mut next, mut coming_from) = match start_type {
        Direction::SE => (start_coord.east(), Direction::West),
        Direction::SW => (start_coord.south(), Direction::North),
        Direction::NS => (start_coord.north().unwrap(), Direction::South),
        _ => panic!("Wut?"),
    };
    loop_path.push(next);

    while next != start_coord {
        let cur = grid.at((next.y, next.x));
        (next, coming_from) = match (cur, coming_from) {
            (Direction::NS, Direction::South) => (next.north().unwrap(), Direction::South),
            (Direction::NS, Direction::North) => (next.south(), Direction::North),
            (Direction::EW, Direction::West) => (next.east(), Direction::West),
            (Direction::EW, Direction::East) => (next.west().unwrap(), Direction::East),
            (Direction::NW, Direction::North) => (next.west().unwrap(), Direction::East),
            (Direction::NW, Direction::West) => (next.north().unwrap(), Direction::South),
            (Direction::NE, Direction::East) => (next.north().unwrap(), Direction::South),
            (Direction::NE, Direction::North) => (next.east(), Direction::West),
            (Direction::SE, Direction::South) => (next.east(), Direction::West),
            (Direction::SE, Direction::East) => (next.south(), Direction::North),
            (Direction::SW, Direction::West) => (next.south(), Direction::North),
            (Direction::SW, Direction::South) => (next.west().unwrap(), Direction::East),
            _ => panic!("Impossible"),
        };
        loop_path.push(next);
    }
    //shoelace formula
    let mut area: isize = 0;
    let n = loop_path.len() as isize;
    for w in loop_path.windows(2) {
        area += (w[0].y * w[1].x) as isize;
        area -= (w[0].x * w[1].y) as isize;
    }
    let area = isize::abs(area) / 2;

    //find number of tiles inside
    (area - (n / 2) + 1) as u64
}

fn get_start_type(start: Coord, grid: &Grid<Direction>) -> Direction {
    let north_type = start.north().map(|c| grid.at((c.y, c.x)));
    let south_type = match start.y {
        y if y < grid.height() - 1 => Some(start.south()),
        _ => None,
    }
    .map(|c| grid.at((c.y, c.x)));
    let east_type = match start.x {
        x if x < grid.width() - 1 => Some(start.east()),
        _ => None,
    }
    .map(|c| grid.at((c.y, c.x)));
    let west_type = start.west().map(|c| grid.at((c.y, c.x)));

    let entry_from_north = if let Some(north_type) = north_type {
        north_type == Direction::NS || north_type == Direction::SE || north_type == Direction::SW
    } else {
        false
    };
    let entry_from_south = if let Some(south_type) = south_type {
        south_type == Direction::NS || south_type == Direction::NE || south_type == Direction::NW
    } else {
        false
    };
    let entry_from_west = if let Some(west_type) = west_type {
        west_type == Direction::EW || west_type == Direction::NE || west_type == Direction::SE
    } else {
        false
    };
    let entry_from_east = if let Some(east_type) = east_type {
        east_type == Direction::EW || east_type == Direction::NW || east_type == Direction::SW
    } else {
        false
    };
    let start_type = if entry_from_north && entry_from_south {
        Direction::NS
    } else if entry_from_north && entry_from_west {
        Direction::NW
    } else if entry_from_north && entry_from_east {
        Direction::NE
    } else if entry_from_south && entry_from_west {
        Direction::SW
    } else if entry_from_south && entry_from_east {
        Direction::SE
    } else if entry_from_west && entry_from_east {
        Direction::EW
    } else {
        panic!("Somethings wrong")
    };
    start_type
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    y: usize,
    x: usize,
}

impl Coord {
    fn south(&self) -> Coord {
        Coord {
            y: self.y + 1,
            x: self.x,
        }
    }

    fn east(&self) -> Coord {
        Coord {
            y: self.y,
            x: self.x + 1,
        }
    }

    fn north(&self) -> Option<Coord> {
        if self.y == 0 {
            return None;
        }
        Some(Coord {
            y: self.y - 1,
            x: self.x,
        })
    }

    fn west(&self) -> Option<Coord> {
        if self.x == 0 {
            return None;
        }
        Some(Coord {
            y: self.y,
            x: self.x - 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2023_10_ep1() {
        assert_eq!(8, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_10_ep2() {
        assert_eq!(4, part2(&get_test_input_part(YEAR, DAY, 2)));
    }

    #[test]
    fn t2023_10_rp1() {
        assert_eq!(6738, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_10_rp2() {
        assert_eq!(579, part2(&get_input(YEAR, DAY)));
    }
}
