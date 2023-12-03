use aocshared::*;

const YEAR: i32 = 2023;
const DAY: u32 = 03;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let grid = get_lines_as_grid_char(data);
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut prev = &'.';
    let mut numbers: Vec<Vec<&char>> = vec![];
    for (y, row) in grid.iter().enumerate() {
        let mut adjacent = false;
        let mut number: Vec<&char> = vec![];
        for (x, item) in row.iter().enumerate() {
            // Test current location
            if item.is_ascii_digit() {
                if !adjacent {
                    adjacent |= get_adj_points((y, x), (grid_width, grid_height), true)
                        .iter()
                        .fold(false, |adj, point| {
                            adj || (grid[point.0][point.1] != '.'
                                && !grid[point.0][point.1].is_ascii_digit())
                        });
                }
                number.push(item);
            }
            // Close the number
            if !item.is_ascii_digit() && prev.is_ascii_digit() {
                if adjacent {
                    numbers.push(number);
                }
                adjacent = false;
                number = vec![];
            }
            prev = item;
        }
        // Close the number
        if prev.is_ascii_digit() && adjacent {
            numbers.push(number);
        }
    }
    numbers.iter().fold(0, |acc, part| {
        acc + part
            .iter()
            .map(|c| *c)
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    })
}

fn part2(data: &String) -> u64 {
    let grid = get_lines_as_grid_char(data);
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut gear_ratios = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if *item == '*' {
                let points = get_adj_points((y, x), (grid_width, grid_height), true);
                let numbers = points
                    .iter()
                    .fold(vec![], |mut adj_digits, point| {
                        if grid[point.0][point.1].is_ascii_digit() {
                            adj_digits.push(point);
                        }
                        adj_digits
                    })
                    .iter()
                    .fold(vec![], |mut numbers, (y, x)| {
                        let mut digits = vec![];
                        digits.push((*y, *x));
                        let mut lx: i32 = *x as i32 - 1;
                        while lx >= 0 && grid[*y][lx as usize].is_ascii_digit() {
                            digits.insert(0, (*y, lx as usize));
                            lx -= 1;
                        }
                        let mut rx = x + 1;
                        while rx < grid_width && grid[*y][rx].is_ascii_digit() {
                            digits.push((*y, rx));
                            rx += 1;
                        }
                        numbers.push(digits);
                        numbers
                    });
                let mut deduped: Vec<Vec<(usize, usize)>> = vec![];
                for digits in numbers {
                    let mut found = false;
                    for value in deduped.iter() {
                        if digits[0] == value[0] {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        deduped.push(digits);
                    }
                }
                if deduped.len() == 2 {
                    let parts = deduped
                        .iter()
                        .map(|number| {
                            number
                                .iter()
                                .map(|p| grid[p.0][p.1])
                                .collect::<String>()
                                .parse::<u64>()
                                .unwrap()
                        })
                        .collect::<Vec<u64>>();
                    gear_ratios.push(parts[0] * parts[1]);
                }
            }
        }
    }
    gear_ratios.iter().fold(0, |acc, g| acc + g)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2023_03_ep1() {
        assert_eq!(4361, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_03_ep2() {
        assert_eq!(467835, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_03_rp1() {
        assert_eq!(531932, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_03_rp2() {
        assert_eq!(73646890, part2(&get_input(YEAR, DAY)));
    }
}
