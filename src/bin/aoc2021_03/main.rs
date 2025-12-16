use aocshared::aoc::aoc::*;

const YEAR: i32 = 2021;
const DAY: u32 = 03;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    let input = get_lines_as_vec_u32(data);
    let input_len = input[0].len();
    let mut col_0s = vec![0; input_len];
    let mut col_1s = vec![0; input_len];
    for entry in &input {
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
    (gamma * epsilon) as i32
}

fn part2(data: &String) -> i32 {
    let input = get_lines_as_vec_chars(data);
    let input_len = input[0].len();
    println!("Input Len: {}", input_len);

    let mut ovalues = input.clone();
    for idx in 0..input_len {
        if ovalues.len() == 1 {
            break;
        }
        let mut zero_count = 0;
        let mut one_count = 0;
        for entry in &ovalues {
            if 0 == entry[idx].to_digit(10).unwrap() {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        let test = if one_count >= zero_count { 1 } else { 0 };
        ovalues = ovalues
            .iter()
            .filter(|entry| test == entry[idx].to_digit(10).unwrap())
            .cloned()
            .collect::<Vec<Vec<char>>>();
    }

    let mut cvalues = input.clone();
    for idx in 0..input_len {
        if cvalues.len() == 1 {
            break;
        }
        let mut zero_count = 0;
        let mut one_count = 0;
        for entry in &cvalues {
            if 0 == entry[idx].to_digit(10).unwrap() {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        let test = if one_count >= zero_count { 0 } else { 1 };
        cvalues = cvalues
            .iter()
            .filter(|entry| test == entry[idx].to_digit(10).unwrap())
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
    (oxygen * co2) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2021_03_ep1() {
        assert_eq!(198, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_03_ep2() {
        assert_eq!(230, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_03_rp1() {
        assert_eq!(3813416, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_03_rp2() {
        assert_eq!(2990784, part2(&get_input(YEAR, DAY)));
    }
}
