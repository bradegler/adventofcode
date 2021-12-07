use aocshared::get_input;

const YEAR: i32 = 2021;
const DAY: u32 = 07;

fn main() {
    let i = get_input(YEAR, DAY);
    part1(&i);
    part2(&i);
}

fn part1(data: &String) -> i32 {
    println!("Part 1");
    let start = data
        .split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let max = start.iter().max().unwrap();
    let mut fuel_costs = vec![0i32; *max as usize + 1];
    for idx in 0..=*max {
        fuel_costs[idx as usize] = start.iter().map(|x| (x - idx).abs()).sum::<i32>();
    }
    let mut min = i32::max_value();
    for idx in 0..fuel_costs.len() {
        let fc = fuel_costs[idx];
        if fc < min {
            min = fc;
        }
    }
    let result = min;
    println!("Part 1 Result: {}", result);
    return result.try_into().unwrap();
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    let start = data
        .split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let max = start.iter().max().unwrap();
    let mut fuel_costs = vec![0i32; *max as usize + 1];
    for idx in 0..=*max {
        fuel_costs[idx as usize] = start.iter().map(|x| fuel((x - idx).abs())).sum::<i32>();
    }
    let mut min = i32::max_value();
    for idx in 0..fuel_costs.len() {
        let fc = fuel_costs[idx];
        if fc < min {
            min = fc;
        }
    }
    let result = min;
    println!("Part 2 Result: {}", result);
    return result;
}

fn fuel(distance: i32) -> i32 {
    (distance * (distance + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_07_ep1() {
        assert_eq!(37, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_07_ep2() {
        assert_eq!(168, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_07_rp1() {
        assert_eq!(356922, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_07_rp2() {
        assert_eq!(100347031, part2(&get_input(YEAR, DAY)));
    }
}
