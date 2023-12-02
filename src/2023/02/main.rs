use aocshared::*;

const YEAR: i32 = 2023;
const DAY: u32 = 02;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let lines = get_lines_as_strs(data)
        .into_iter()
        .map(|l| l.replace("Game ", ""))
        .map(|l| l.replace(";", ","));
    let rgb = (12, 13, 14);
    lines.fold(0, |acc, line| {
        let mut split = line.split(":");
        let id = split.next().unwrap().parse::<u64>().unwrap();
        let cubes = split.next().unwrap().split(",");
        match cubes.fold(true, |acc, cube| {
            let shown = cube.trim().split(" ").collect::<Vec<&str>>();
            let count = shown[0].parse::<u64>().unwrap();
            let p = match shown[1] {
                "blue" => count <= rgb.2,
                "green" => count <= rgb.1,
                "red" => count <= rgb.0,
                _ => false,
            };
            p && acc
        }) {
            true => acc + id,
            false => acc,
        }
    })
}

fn part2(data: &String) -> u64 {
    let lines = get_lines_as_strs(data)
        .into_iter()
        .map(|l| l.replace("Game ", ""))
        .map(|l| l.replace(";", ","));
    lines.fold(0, |acc, line| {
        let cubes = line.split(":").nth(1).unwrap().split(",");
        let res = cubes.fold((0, 0, 0), |rgb, cube| {
            let mut shown = cube.trim().split(" ");
            let count = shown.next().unwrap().parse::<u64>().unwrap();
            match shown.next().unwrap() {
                "blue" if count > rgb.2 => (rgb.0, rgb.1, count),
                "green" if count > rgb.1 => (rgb.0, count, rgb.2),
                "red" if count > rgb.0 => (count, rgb.1, rgb.2),
                &_ => rgb,
            }
        });
        acc + res.0 * res.1 * res.2
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2023_02_ep1() {
        assert_eq!(8, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_02_ep2() {
        assert_eq!(2286, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_02_rp1() {
        assert_eq!(2101, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_02_rp2() {
        assert_eq!(58269, part2(&get_input(YEAR, DAY)));
    }
}
