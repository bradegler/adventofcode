use aocshared::{aoc::aoc::*, grid::grid::Grid};
use itertools::Itertools;

const YEAR: i32 = 2024;
const DAY: u32 = 08;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let content = Grid::new(get_lines_as_grid_char(data));
    let mut antinodes = Grid::new(vec![vec![0; content.width()]; content.height()]);

    let nodes = content
        .into_iter()
        .filter(|(_, _, v)| *v != '.')
        .map(|(x, y, v)| (x as i32, y as i32, v))
        .collect_vec();

    for node in nodes.clone() {
        println!("{:?}", node);
        // find all nodes of the same frequency
        let same_frequency_nodes = nodes
            .iter()
            .filter(|n| n.2 == node.2)
            .filter(|n| n.0 != node.0 && n.1 != node.1)
            .collect_vec();
        println!("same_frequency_nodes: {:?}", same_frequency_nodes);
        // calculate distancce between nodes
        let distances = same_frequency_nodes
            .iter()
            .map(|n| ((n.0 - node.0), (n.1 - node.1)))
            .collect_vec();
        println!("distances: {:?}", distances);
        // calculate the antinode as 2x distance to node
        for distance in distances {
            let antinode = (node.0 + distance.0 * 2, node.1 + distance.1 * 2);
            println!("antinode: {:?}", antinode);
            if antinodes.in_bounds(antinode) {
                antinodes.set((antinode.0 as usize, antinode.1 as usize), 1);
            }
        }
    }
    antinodes.into_iter().map(|(_, _, i)| i).sum()
}

fn part2(data: &String) -> u64 {
    let content = Grid::new(get_lines_as_grid_char(data));
    let mut antinodes = Grid::new(vec![vec![0; content.width()]; content.height()]);

    let nodes = content
        .into_iter()
        .filter(|(_, _, v)| *v != '.')
        .map(|(x, y, v)| (x as i32, y as i32, v))
        .collect_vec();

    for node in nodes.clone() {
        println!("{:?}", node);
        // find all nodes of the same frequency
        let same_frequency_nodes = nodes
            .iter()
            .filter(|n| n.2 == node.2)
            .filter(|n| n.0 != node.0 && n.1 != node.1)
            .collect_vec();
        println!("same_frequency_nodes: {:?}", same_frequency_nodes);
        // calculate distance between nodes
        let distances = same_frequency_nodes
            .iter()
            .map(|n| ((n.0 - node.0), (n.1 - node.1)))
            .collect_vec();
        println!("distances: {:?}", distances);
        // calculate the antinode as 2x distance to node
        for distance in distances {
            antinodes.set(
                (
                    node.0 as usize + distance.0 as usize,
                    node.1 as usize + distance.1 as usize,
                ),
                1,
            );
            let mut antinode = (node.0 + distance.0 * 2, node.1 + distance.1 * 2);
            while antinodes.in_bounds(antinode) {
                println!("antinode: {:?}", antinode);
                antinodes.set((antinode.0 as usize, antinode.1 as usize), 1);
                antinode = (antinode.0 + distance.0 * 2, antinode.1 + distance.1 * 2);
            }
        }
    }
    antinodes.into_iter().map(|(_, _, i)| i).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2024_08_ep1() {
        assert_eq!(14, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_08_ep2() {
        assert_eq!(9, part2(&get_test_input_part(YEAR, DAY, 2)));
        assert_eq!(34, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_08_rp1() {
        assert_eq!(392, part1(&get_input(YEAR, DAY)));
    }

    // #[test]
    // fn t2024_08_rp2() {
    //     assert_eq!(0, part2(&get_input(YEAR, DAY)));
    // }
}
