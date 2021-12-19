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
        println!("{} {}", ins, amt);
        match ins.as_str() {
            "F" => {
                by += wy * amt;
                bx += wx * amt;
            }
            "N" => wy += amt,
            "S" => wy -= amt,
            "E" => wx += amt,
            "W" => wx -= amt,

            "L" => {
                let wp = rotate("L", amt, (wx, wy));
                wx = wp.0;
                wy = wp.1;
            }
            "R" => {
                let wp = rotate("R", amt, (wx, wy));
                wx = wp.0;
                wy = wp.1;
            }
            _ => panic!("Invalid instruction {}", ins),
        }
        println!("WP: {} {} - Boat: {}, {}", wx, wy, bx, by);
    }
    // Manhattan distance
    bx.abs() + by.abs()
}

fn rotate(direction: &str, angle: i32, (x, y): (i32, i32)) -> (i32, i32) {
    match direction {
        "L" => match angle {
            90 => (-y, x),
            180 => (-x, -y),
            270 => (y, -x),
            _ => unreachable!(),
        },
        "R" => match angle {
            90 => (y, -x),
            180 => (-x, -y),
            270 => (-y, x),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
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
        assert_eq!(42013, part2(&get_input(YEAR, DAY)));
    }

    #[test]
    fn test_wp_rotation() {
        assert_eq!((4, -10), rotate("R", 90, (10, 4)));
        assert_eq!((-10, -4), rotate("R", 180, (10, 4)));
        assert_eq!((-4, 10), rotate("R", 270, (10, 4)));

        assert_eq!((-4, 10), rotate("L", 90, (10, 4)));
        assert_eq!((-10, -4), rotate("L", 180, (10, 4)));
        assert_eq!((4, -10), rotate("L", 270, (10, 4)));
    }
}
