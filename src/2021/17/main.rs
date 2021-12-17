use aocshared::*;
use regex::Regex;

const YEAR: i32 = 2021;
const DAY: u32 = 17;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    /*
    The probe's x position increases by its x velocity.
    The probe's y position increases by its y velocity.
    Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
    Due to gravity, the probe's y velocity decreases by 1.
    */
    let (x_range, y_range) = get_ranges(data);
    let mut max = 0;
    for velx in 0..1000 {
        for vely in -500..1000 {
            // Launch the probe
            let (mut x, mut y) = (0, 0);
            let (mut vx, mut vy) = (velx, vely);
            let mut max_height = 0;
            while x <= x_range.1 && y >= y_range.0 {
                if y > max_height {
                    max_height = y;
                }
                // Move the probe
                x += vx;
                y += vy;
                // Adjust the velocity
                if vx > 0 {
                    vx -= 1;
                }
                vy -= 1;
                // Check if the probe is in the target area
                if x >= x_range.0 && x <= x_range.1 && y >= y_range.0 && y <= y_range.1 {
                    if max_height > max {
                        max = max_height;
                    }
                    break;
                }
            }
        }
    }
    max
}

fn get_ranges(data: &String) -> ((i32, i32), (i32, i32)) {
    let pattern = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$").unwrap();
    let input = data.replace("\n", "");
    let captures = pattern.captures(&input).unwrap();
    let x_min = captures[1].parse::<i32>().unwrap();
    let x_max = captures[2].parse::<i32>().unwrap();
    let y_min = captures[3].parse::<i32>().unwrap();
    let y_max = captures[4].parse::<i32>().unwrap();
    let x_range = (x_min, x_max);
    let y_range = (y_min, y_max);
    (x_range, y_range)
}

fn part2(data: &String) -> i32 {
    let (x_range, y_range) = get_ranges(data);
    let mut cnt = 0;
    for velx in 0..1000 {
        for vely in -500..1000 {
            // Launch the probe
            let (mut x, mut y) = (0, 0);
            let (mut vx, mut vy) = (velx, vely);
            let mut max_height = 0;
            while x <= x_range.1 && y >= y_range.0 {
                if y > max_height {
                    max_height = y;
                }
                // Move the probe
                x += vx;
                y += vy;
                // Adjust the velocity
                if vx > 0 {
                    vx -= 1;
                }
                vy -= 1;
                // Check if the probe is in the target area
                if x >= x_range.0 && x <= x_range.1 && y >= y_range.0 && y <= y_range.1 {
                    cnt += 1;
                    break;
                }
            }
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_17_ep1() {
        assert_eq!(45, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_17_ep2() {
        assert_eq!(112, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_17_rp1() {
        assert_eq!(13203, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_17_rp2() {
        assert_eq!(5644, part2(&get_input(YEAR, DAY)));
    }
}
