use aocf::Aoc;

const YEAR: i32 = 2021;
const DAY: u32 = 3;

fn get_input(year: i32, day: u32) -> String {
    println!("Advent of Code {} - Day {}", year, day);

    let mut aoc = Aoc::new().year(Some(year)).day(Some(day)).init().unwrap();

    // Get the brief
    let brief = aoc.get_brief(false);
    if let Ok(brief) = brief {
        println!("{}", brief);
    }

    // Get input data (don't force)
    if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        panic!("No input found");
    }
}

fn main() {
    let i = get_input(YEAR, DAY);
    part1(&i);
    part2(&i);
}

fn part1(data: &String) -> i32 {
    println!("Part 1");
    const RADIX: u32 = 10;
    let report = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let as_chars = report
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(RADIX).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let input_len = as_chars[0].len();
    println!("Input Len: {}", input_len);
    let mut col_0s = vec![0; input_len];
    let mut col_1s = vec![0; input_len];
    for entry in &as_chars {
        for idx in 0..input_len {
            if 0 == entry[idx] {
                col_0s[idx] += 1;
            } else {
                col_1s[idx] += 1;
            }
        }
    }
    let mut gstr = "".to_owned();
    let mut estr = "".to_owned();
    for idx in 0..input_len {
        if col_0s[idx] > col_1s[idx] {
            gstr.push_str("0");
            estr.push_str("1");
        } else {
            gstr.push_str("1");
            estr.push_str("0");
        }
    }

    let gamma = isize::from_str_radix(&gstr[..], 2).unwrap();
    let epsilon = isize::from_str_radix(&estr[..], 2).unwrap();
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);

    let result = gamma * epsilon;
    println!("Part 1 Result: {}", result);
    return result.try_into().unwrap();
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    const RADIX: u32 = 10;
    let report = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let as_chars = report
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let input_len = as_chars[0].len();
    println!("Input Len: {}", input_len);

    let mut ovalues = as_chars.clone();
    for idx in 0..input_len {
        if ovalues.len() == 1 {
            break;
        }
        let mut zero_count = 0;
        let mut one_count = 0;
        for entry in &ovalues {
            if 0 == entry[idx].to_digit(RADIX).unwrap() {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        let test = if one_count >= zero_count { 1 } else { 0 };
        ovalues = ovalues
            .iter()
            .filter(|entry| test == entry[idx].to_digit(RADIX).unwrap())
            .cloned()
            .collect::<Vec<Vec<char>>>();
    }

    let mut cvalues = as_chars.clone();
    for idx in 0..input_len {
        if cvalues.len() == 1 {
            break;
        }
        let mut zero_count = 0;
        let mut one_count = 0;
        for entry in &cvalues {
            if 0 == entry[idx].to_digit(RADIX).unwrap() {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        let test = if one_count >= zero_count { 0 } else { 1 };
        cvalues = cvalues
            .iter()
            .filter(|entry| test == entry[idx].to_digit(RADIX).unwrap())
            .cloned()
            .collect::<Vec<Vec<char>>>();
    }

    assert_eq!(ovalues.len(), 1);
    assert_eq!(cvalues.len(), 1);
    let oxygen_generator_rating = &ovalues[0];
    let ogr: String = oxygen_generator_rating.iter().collect();
    let co2_scrubber_rating = &cvalues[0];
    let csr: String = co2_scrubber_rating.into_iter().collect();

    let oxygen = isize::from_str_radix(&ogr[..], 2).unwrap();
    let co2 = isize::from_str_radix(&csr[..], 2).unwrap();
    println!("Oxygen: {}", oxygen);
    println!("CO2: {}", co2);

    let result = oxygen * co2;
    println!("Part 2 Result: {}", result);
    return result.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_part1() {
        let data = read_to_string("test.txt").unwrap();
        let count = part1(&data);
        assert_eq!(198, count);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("test.txt").unwrap();
        let count = part2(&data);
        assert_eq!(230, count);
    }

    #[test]
    fn real_part1() {
        let data = get_input(YEAR, DAY);
        let count = part1(&data);
        assert_eq!(3813416, count);
    }

    #[test]
    fn real_part2() {
        let data = get_input(YEAR, DAY);
        let count = part2(&data);
        assert_eq!(2990784, count);
    }
}
