use aocshared::get_input;

const YEAR: i32 = 2021;
const DAY: u32 = 06;

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

    let days = 80;
    let mut fish = start;
    for range in 1..=days {
        let mut new_fish = Vec::<i32>::new();
        fish = fish
            .iter()
            .map(|x| {
                let result = match *x {
                    0 => {
                        new_fish.push(8);
                        6
                    }
                    _ => x - 1,
                };
                result
            })
            .collect::<Vec<i32>>();
        fish.append(&mut new_fish);
    }

    let result = fish.len() as i32;
    println!("Part 1 Result: {}", result);
    return result;
}

fn part2(data: &String) -> i64 {
    println!("Part 2");
    let start = data
        .split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let days = 256;
    let mut fish_days = vec![0; 9];
    start.iter().for_each(|x| fish_days[*x as usize] += 1);
    for _ in 1..=days {
        let mut new_fish_days = vec![0; 9];
        new_fish_days[0] = fish_days[1];
        new_fish_days[1] = fish_days[2];
        new_fish_days[2] = fish_days[3];
        new_fish_days[3] = fish_days[4];
        new_fish_days[4] = fish_days[5];
        new_fish_days[5] = fish_days[6];
        new_fish_days[6] = fish_days[0] + fish_days[7];
        new_fish_days[7] = fish_days[8];
        new_fish_days[8] = fish_days[0];
        fish_days = new_fish_days;
    }
    let result = fish_days.iter().sum::<i64>();
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
        assert_eq!(17673235392090, part2(&get_input(YEAR, DAY)));
    }
}
