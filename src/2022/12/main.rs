use aocshared::*;

const YEAR: i32 = 2022;
const DAY: u32 = 12;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let grid_char = get_lines_as_grid_char(data);
    let mut grid = grid_char
        .iter()
        .map(|row| row.iter().map(|c| *c as u32 - 97).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    let grid_size = (grid[0].len(), grid.len());
    let mut distances = vec![vec![u32::MAX; grid_size.0]; grid_size.1];
    let mut parents = vec![vec![(0, 0); grid_size.0]; grid_size.1];
    let mut start = (0, 0);
    let mut dest = (0, 0);
    for (y, row) in grid_char.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 'S' {
                start = (y, x);
            }
            if *value == 'E' {
                dest = (y, x);
            }
        }
    }
    println!("{},{}->{},{}", start.0, start.1, dest.0, dest.1);
    grid[start.0][start.1] = 0;
    grid[dest.0][dest.1] = 0;
    distances[start.0][start.1] = 0;
    let current = start;

    loop {
        let points = get_adj_points(current, grid_size, false);
        let current_value = grid[current.0][current.1];
        let current_dist = distances[current.0][current.1];
        for point in points {
            let point_value = grid[point.0][point.1];
            let new_dist = current_dist + 1;
            // can visit node
            if point_value <= current_value + 1 {
                let point_dist = distances[point.0][point.1];
                if point_dist > new_dist {
                    distances[point.0][point.1] = new_dist;
                    parents[point.0][point.1] = current;
                }
            }
            // find next current as the point with the shortest distance
        }
        break;
    }

    print_grid(&grid);
    0
}

fn part2(_data: &String) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2022_12_ep1() {
        assert_eq!(31, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_12_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_12_rp1() {
        assert_eq!(0, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_12_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
