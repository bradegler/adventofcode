use aocshared::*;

const YEAR: i32 = 2020;
const DAY: u32 = 12;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn steer(current_direction: i32, angle: i32) -> i32 {
    let mut new_direction = current_direction + angle;
    if new_direction < 0 {
        new_direction = 360 + new_direction;
    }
    if new_direction >= 360 {
        new_direction = new_direction - 360;
    }
    new_direction
}

fn part1(data: &String) -> i32 {
    let input = get_lines_as_strs(data);
    let motions = input.iter().map(|l| {
        let cs = l.chars().collect::<Vec<char>>();
        let ins = cs[0..1].iter().collect::<String>();
        let amt = cs[1..].iter().collect::<String>().parse::<i32>().unwrap();
        (ins, amt)
    });
    let mut y = 0;
    let mut x = 0;
    let mut boat_direction = 0;
    for (ins, amt) in motions {
        match ins.as_str() {
            "F" => match boat_direction {
                0 => x += amt,
                90 => y += amt,
                180 => x -= amt,
                270 => y -= amt,

                _ => panic!("Invalid direction {}", boat_direction),
            },
            "N" => y += amt,
            "S" => y -= amt,
            "E" => x += amt,
            "W" => x -= amt,

            "L" => boat_direction = steer(boat_direction, amt),
            "R" => boat_direction = steer(boat_direction, -1 * amt),
            _ => panic!("Invalid instruction {}", ins),
        }
    }
    // Manhattan distance
    x.abs() + y.abs()
}

fn part2(data: &String) -> i32 {
    let input = get_lines_as_strs(data);
    let motions = input.iter().map(|l| {
        let cs = l.chars().collect::<Vec<char>>();
        let ins = cs[0..1].iter().collect::<String>();
        let amt = cs[1..].iter().collect::<String>().parse::<i32>().unwrap();
        (ins, amt)
    });
    let mut by = 0;
    let mut bx = 0;
    let mut wy = 1;
    let mut wx = 10;
    for (ins, amt) in motions {
        match ins.as_str() {
            "F" => {
                by += ((by - wy) as i32).abs() * amt;
                bx += ((bx - wx) as i32).abs() * amt;
                wy = by + 1;
                wx = bx + 10;
            }
            "N" => wy += amt,
            "S" => wy -= amt,
            "E" => wx += amt,
            "W" => wx -= amt,

            "L" => match amt {
                90 => {
                    wx = bx - wx;
                    wy = by - wy;
                }
                180 => {
                    wx = bx + wx;
                }
                270 => {
                    wy = by + (wx - bx).abs();
                    wx = by + (wy - by).abs();
                }
                _ => panic!("Invalid direction {}", amt),
            },
            "R" => match amt {
                90 => {
                    wy = by - (wx - bx).abs();
                    wx = bx - (wy - by).abs();
                }
                180 => {
                    wx = bx - wx;
                }
                270 => {
                    wx = bx + wx;
                    wy = by + wy;
                }
                _ => panic!("Invalid direction {}", amt),
            },
            _ => panic!("Invalid instruction {}", ins),
        }
        println!("WP: {} {} - Boat: {}, {}", wx, wy, bx, by);
    }
    // Manhattan distance
    bx.abs() + by.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2020_12_ep1() {
        assert_eq!(25, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_12_ep2() {
        assert_eq!(286, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_12_rp1() {
        assert_eq!(590, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_12_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
