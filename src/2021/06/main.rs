use aocshared::get_input;

const YEAR: i32 = 2021;
const DAY: u32 = 06;

fn main() {
    let i = get_input(YEAR, DAY);
    part1(&i);
    part2(&i);
}

fn spawn_count(input: &str, days: i32) -> i64 {
    let start = input
        .split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut fish_days = vec![0; 9];
    start.iter().for_each(|x| fish_days[*x as usize] += 1);
    for _ in 1..=days {
        let mut new_fish_days = vec![0; 9];
        for i in 0..8 {
            if i == 0 {
                new_fish_days[6] = fish_days[i];
                new_fish_days[8] = fish_days[i];
            }
            new_fish_days[i] = new_fish_days[i] + fish_days[i + 1];
        }
        fish_days = new_fish_days;
    }
    fish_days.iter().sum::<i64>()
}

fn part1(data: &String) -> i64 {
    println!("Part 1");
    let result = spawn_count(data, 80);
    println!("Part 1 Result: {}", result);
    return result;
}

fn part2(data: &String) -> i64 {
    println!("Part 2");
    let result = spawn_count(data, 256);
    println!("Part 2 Result: {}", result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_06_ep1() {
        assert_eq!(5934, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_06_ep2() {
        assert_eq!(26984457539, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_06_rp1() {
        assert_eq!(395627, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_06_rp2() {
        assert_eq!(1767323539209, part2(&get_input(YEAR, DAY)));
    }
}
