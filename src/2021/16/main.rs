use aocshared::*;

const YEAR: i32 = 2021;
const DAY: u32 = 16;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> usize {
    let bin = convert(&data.chars().collect());
    let (p, _) = parse_packet(&bin.chars().collect(), 0);
    sum_versions(&vec![p])
}

fn part2(data: &String) -> usize {
    let bin = convert(&data.chars().collect());
    let (p, _) = parse_packet(&bin.chars().collect(), 0);
    calculate_packet(&p)
}

struct Packet {
    version: usize,
    type_id: usize,
    literal: Option<usize>,
    sub_packets: Option<Vec<Packet>>,
}

fn bin_to_int(bin: &str) -> usize {
    usize::from_str_radix(bin, 2).unwrap()
}

fn parse_packet(packet: &Vec<char>, mut packet_start: usize) -> (Packet, usize) {
    let version = bin_to_int(
        &*packet[packet_start..packet_start + 3]
            .iter()
            .collect::<String>(),
    );
    let type_id = bin_to_int(
        &*packet[packet_start + 3..packet_start + 6]
            .iter()
            .collect::<String>(),
    );
    println!(
        "PacketStart: {}, Version: {}, Type: {}",
        packet_start, version, type_id
    );
    packet_start += 6;
    if type_id == 4 {
        // Literal packet
        let mut lvalue: String = "".to_string();
        loop {
            let con = packet[packet_start];
            lvalue.push_str(
                &*packet[packet_start + 1..packet_start + 5]
                    .iter()
                    .collect::<String>(),
            );
            packet_start += 5;
            if con == '0' {
                break;
            }
        }
        let literal = bin_to_int(&lvalue);
        return (
            Packet {
                version: version,
                type_id: type_id,
                literal: Some(literal),
                sub_packets: None,
            },
            packet_start,
        );
    } else {
        // Operator packet
        let length_type_id = bin_to_int(&*packet[packet_start].to_string());
        packet_start += 1;
        let mut sub_packets = vec![];
        if length_type_id == 0 {
            // 15 bit value that represents the subpacket length
            let sp_length = bin_to_int(
                &*packet[packet_start..packet_start + 15]
                    .iter()
                    .collect::<String>(),
            );
            println!("sp_length: {}", sp_length);
            packet_start += 15;
            let loop_end = packet_start + sp_length;
            while packet_start < loop_end {
                let (subpacket, ns) = parse_packet(packet, packet_start);
                packet_start = ns;
                sub_packets.push(subpacket);
            }
        } else {
            // 11 bit value that represents the number of subpackets
            let sp_num = bin_to_int(
                &*packet[packet_start..packet_start + 11]
                    .iter()
                    .collect::<String>(),
            );
            println!("sp_num: {}", sp_num);
            packet_start += 11;
            for _ in 0..sp_num {
                let (subpacket, ns) = parse_packet(packet, packet_start);
                packet_start = ns;
                sub_packets.push(subpacket);
            }
        }
        return (
            Packet {
                version: version,
                type_id: type_id,
                literal: None,
                sub_packets: Some(sub_packets),
            },
            packet_start,
        );
    }
}

fn hex_to_bin(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn convert(input: &Vec<char>) -> String {
    let mut output = String::new();
    for c in input {
        output.push_str(hex_to_bin(*c));
    }
    output
}

fn sum_versions(packets: &Vec<Packet>) -> usize {
    let mut sum = 0;
    for p in packets {
        sum += p.version;
        if p.sub_packets.is_some() {
            sum += sum_versions(&p.sub_packets.as_ref().unwrap());
        }
    }
    sum
}

fn calculate_packet(packet: &Packet) -> usize {
    match packet.type_id {
        4 => packet.literal.unwrap(),
        0 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(|sp| calculate_packet(&sp))
            .sum::<usize>(),
        1 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(|sp| calculate_packet(&sp))
            .product::<usize>(),
        2 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(|sp| calculate_packet(&sp))
            .min()
            .unwrap(),
        3 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(|sp| calculate_packet(&sp))
            .max()
            .unwrap(),
        5 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(|sp| calculate_packet(&sp))
            .reduce(|a, b| if a > b { 1 } else { 0 })
            .unwrap(),
        6 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(|sp| calculate_packet(&sp))
            .reduce(|a, b| if a < b { 1 } else { 0 })
            .unwrap(),
        7 => packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(|sp| calculate_packet(&sp))
            .reduce(|a, b| if a == b { 1 } else { 0 })
            .unwrap(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn test_hex_to_bin() {
        assert_eq!("0000", hex_to_bin('0'));
        assert_eq!("0001", hex_to_bin('1'));
        assert_eq!("0010", hex_to_bin('2'));
        assert_eq!("0011", hex_to_bin('3'));
        assert_eq!("0100", hex_to_bin('4'));
        assert_eq!("0101", hex_to_bin('5'));
        assert_eq!("0110", hex_to_bin('6'));
        assert_eq!("0111", hex_to_bin('7'));
        assert_eq!("1000", hex_to_bin('8'));
        assert_eq!("1001", hex_to_bin('9'));
        assert_eq!("1010", hex_to_bin('A'));
        assert_eq!("1011", hex_to_bin('B'));
        assert_eq!("1100", hex_to_bin('C'));
        assert_eq!("1101", hex_to_bin('D'));
        assert_eq!("1110", hex_to_bin('E'));
        assert_eq!("1111", hex_to_bin('F'));
    }

    #[test]
    fn test_convert() {
        assert_eq!(
            "110100101111111000101000",
            convert(&"D2FE28".chars().collect())
        );
        assert_eq!(
            "00111000000000000110111101000101001010010001001000000000",
            convert(&"38006F45291200".chars().collect())
        );
        assert_eq!(
            "11101110000000001101010000001100100000100011000001100000",
            convert(&"EE00D40C823060".chars().collect())
        );
        //assert_eq!(0, part1(&get_test_input(YEAR, DAY)));
    }
    #[test]
    fn t2021_16_ep1() {
        let bin = convert(&"8A004A801A8002F478".chars().collect());
        let (p, _) = parse_packet(&bin.chars().collect(), 0);
        assert_eq!(16, sum_versions(&vec![p]));
        let bin = convert(&"620080001611562C8802118E34".chars().collect());
        let (p, _) = parse_packet(&bin.chars().collect(), 0);
        assert_eq!(12, sum_versions(&vec![p]));
        let bin = convert(&"C0015000016115A2E0802F182340".chars().collect());
        let (p, _) = parse_packet(&bin.chars().collect(), 0);
        assert_eq!(23, sum_versions(&vec![p]));
        let bin = convert(&"A0016C880162017C3686B18A3D4780".chars().collect());
        let (p, _) = parse_packet(&bin.chars().collect(), 0);
        assert_eq!(31, sum_versions(&vec![p]));
    }
    #[test]
    fn test_parse_packet() {
        println!("Test Packet: D2FE28");
        let (p, _) = parse_packet(&"110100101111111000101000".chars().collect(), 0);
        assert_eq!(p.version, 6);
        assert_eq!(p.type_id, 4);
        assert_eq!(p.literal.unwrap(), 2021);
        println!("Test Packet: 38006F45291200");
        let (p, _) = parse_packet(
            &"00111000000000000110111101000101001010010001001000000000"
                .chars()
                .collect(),
            0,
        );
        assert_eq!(p.version, 1);
        assert_eq!(p.type_id, 6);
        assert_eq!(p.sub_packets.unwrap().len(), 2);
        println!("Test Packet: EE00D40C823060");
        let (p, _) = parse_packet(
            &"11101110000000001101010000001100100000100011000001100000"
                .chars()
                .collect(),
            0,
        );
        assert_eq!(p.version, 7);
        assert_eq!(p.type_id, 3);
        assert_eq!(p.sub_packets.unwrap().len(), 3);
    }

    #[test]
    fn t2021_16_ep2() {
        let tests = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for test in tests {
            let bin = convert(&test.0.chars().collect());
            let (p, _) = parse_packet(&bin.chars().collect(), 0);
            assert_eq!(test.1, calculate_packet(&p));
        }
    }

    #[test]
    fn t2021_16_rp1() {
        assert_eq!(854, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_16_rp2() {
        assert_eq!(186189840660, part2(&get_input(YEAR, DAY)));
    }
}
