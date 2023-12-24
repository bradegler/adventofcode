use aocshared::aoc::aoc::*;
use aocshared::grid::grid::{Grid, Point, Visitable};

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
    let contents = get_lines_as_grid_char(data);
    let mut grid = Grid::new(contents);
    let spoint = grid.find(START)[0];
    let mut search = valid_moves(&grid, spoint);
    let mut distance = Grid::new(vec![vec![0; grid.width()]; grid.height()]);
    let mut current_distance = 0;
    while search.len() > 0 {
        let mut next = vec![];
        current_distance += 1;
        for (y, x) in search.into_iter() {
            let visited = grid.is_visited((y, x));
            if !visited {
                grid.visit((y, x));
                distance.set((y, x), current_distance);
                valid_moves(&grid, (y, x))
                    .iter()
                    .for_each(|m| next.push(*m));
            }
        }
        search = next.clone();
    }
    println!();
    distance.print();
    println!();
    grid.print();

    let mut ground_map = grid.clone();
    let ground_points = ground_map.find(GROUND);
    for (y, x) in ground_points {
        if y == 0 || y == ground_map.height() - 1 || x == 0 || x == ground_map.width() - 1 {
            ground_map.set((y, x), 'O');
        }
    }
    let ground_points = ground_map.find(GROUND);
    for (y, x) in ground_points {
        let adj = ground_map.get_adj_points((y, x), false);
    }
    println!();
    ground_map.print();
    println!();

    0
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
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
