use aocshared::*;
use regex::Regex;

const YEAR: i32 = 2020;
const DAY: u32 = 04;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn is_valid_p1(map: &std::collections::HashMap<&str, &str>) -> bool {
    map.contains_key("byr")
        && map.contains_key("iyr")
        && map.contains_key("eyr")
        && map.contains_key("hgt")
        && map.contains_key("hcl")
        && map.contains_key("ecl")
        && map.contains_key("pid")
}

fn is_valid_p2(map: &std::collections::HashMap<&str, &str>) -> bool {
    let eye_colors: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let color_code = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let byr = map
        .get("byr")
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|v| (1920..=2002).contains(v))
        .is_some();
    let iyr = map
        .get("iyr")
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|v| (2010..=2020).contains(v))
        .is_some();
    let eyr = map
        .get("eyr")
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|v| (2020..=2030).contains(v))
        .is_some();
    let hgt = map
        .get("hgt")
        .filter(|v| {
            let (h, unit) = v.split_at(v.len() - 2);
            let h = h.parse::<i32>().unwrap_or(0);
            match unit {
                "cm" => (150..=193).contains(&h),
                "in" => (59..=76).contains(&h),
                _ => false,
            }
        })
        .is_some();
    let hcl = map.get("hcl").filter(|v| color_code.is_match(v)).is_some();
    let ecl = map.get("ecl").filter(|v| eye_colors.contains(v)).is_some();
    let pid = map.get("pid").filter(|v| v.len() == 9).is_some();
    println!("{:?}", map);
    println!(
        "byr={:?}, iyr={:?}, eyr={:?}, hgt={:?}, hcl={:?}, ecl={:?}, pid={:?}",
        byr, iyr, eyr, hgt, hcl, ecl, pid
    );
    byr && iyr && eyr && hgt && hcl && ecl && pid
}

fn part1(data: &String) -> usize {
    data.replace("\n\n", "|")
        .replace("\n", " ")
        .split("|")
        .map(|s| {
            let mut map = std::collections::HashMap::new();
            let segments = s.split(" ");
            for segment in segments {
                let kv = segment.split(":").collect::<Vec<&str>>();
                if kv.len() == 2 {
                    map.insert(kv[0], kv[1]);
                }
            }
            map
        })
        .filter(|m| is_valid_p1(m))
        .count()
}

fn part2(data: &String) -> usize {
    data.replace("\n\n", "|")
        .replace("\n", " ")
        .split("|")
        .map(|s| {
            let mut map = std::collections::HashMap::new();
            let segments = s.split(" ");
            for segment in segments {
                let kv = segment.split(":").collect::<Vec<&str>>();
                if kv.len() == 2 {
                    map.insert(kv[0], kv[1]);
                }
            }
            map
        })
        .filter(|m| is_valid_p2(m))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2020_04_ep1() {
        assert_eq!(2, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_04_ep2_invalid() {
        let invalid = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";
        assert_eq!(0, part2(&invalid.to_string()));
    }
    #[test]
    fn t2020_04_ep2_valid() {
        let valid = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
        assert_eq!(4, part2(&valid.to_string()));
    }

    #[test]
    fn t2020_04_rp1() {
        assert_eq!(235, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_04_rp2() {
        assert_eq!(194, part2(&get_input(YEAR, DAY)));
    }
}
