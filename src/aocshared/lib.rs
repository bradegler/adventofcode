use aocf::Aoc;
use std::fs::read_to_string;

pub fn get_input(year: i32, day: u32) -> String {
    let mut aoc = Aoc::new().year(Some(year)).day(Some(day)).init().unwrap();
    if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        panic!("No input found");
    }
}

pub fn get_test_input(year: i32, day: u32) -> String {
    let file = format!("testdata/{}_{:0>2}.txt", year, day);
    if let Ok(i) = read_to_string(&file) {
        i
    } else {
        panic!("No input found {}", &file);
    }
}

pub fn get_numbers_from_line(line: &str) -> Vec<i32> {
    line.trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

pub fn get_lines_as_numbers(lines: &str) -> Vec<u32> {
    lines.lines().map(|s| s.parse::<u32>().unwrap()).collect()
}

pub fn get_lines_as_numbers_u64(lines: &str) -> Vec<u64> {
    lines.lines().map(|s| s.parse::<u64>().unwrap()).collect()
}

pub fn get_lines_as_strs(lines: &str) -> Vec<&str> {
    lines.lines().collect()
}

pub fn get_lines_as_strs_rm_empty(lines: &str) -> Vec<&str> {
    lines.lines().filter(|l| !l.is_empty()).collect()
}

pub fn get_lines_as_vec_chars(lines: &str) -> Vec<Vec<char>> {
    lines.lines().map(|s| s.chars().collect()).collect()
}

pub fn get_lines_as_vec_u32(lines: &str) -> Vec<Vec<u32>> {
    lines
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn get_lines_as_vec_usize(lines: &str) -> Vec<Vec<usize>> {
    lines
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

// Splits a slice into equal sized chunks.
pub fn chunks<T>(slice: &[T], mut size: usize) -> impl Iterator<Item = &[T]> {
    struct ChunksIterator<'a, I> {
        pub slice: &'a [I],
        pub total: usize,
        pub current: usize,
        pub chunk: usize,
    }
    impl<'a, I> Iterator for ChunksIterator<'a, I> {
        type Item = &'a [I];
        fn next(&mut self) -> Option<&'a [I]> {
            if self.current == self.total {
                None
            } else if self.slice.len() / self.chunk == 1 && self.slice.len() % self.chunk != 0 {
                self.current += 2;
                Some(self.slice)
            } else {
                self.current += 1;
                let (lhs, rhs) = self.slice.split_at(self.chunk);
                self.slice = rhs;
                Some(lhs)
            }
        }
    }
    while slice.len() % size > 1 {
        size += 1;
    }
    let mut len = slice.len() / size;
    if slice.len() % len != 0 {
        len += 1;
    }
    ChunksIterator {
        slice,
        total: len,
        chunk: size,
        current: 0,
    }
}

pub fn print_grid<T>(grid: &Vec<Vec<T>>)
where
    T: std::fmt::Display,
{
    for row in grid {
        for v in row {
            print!("{}", v);
        }
        println!();
    }
}

pub fn transpose(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut transposed = vec![vec![0; grid.len()]; grid[0].len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            transposed[j][i] = *v;
        }
    }
    transposed
}

pub fn transpose_bool(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut transposed = vec![vec![false; grid[0].len()]; grid.len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            transposed[j][i] = *v;
        }
    }
    transposed
}

pub fn get_adj_points(
    (y, x): (usize, usize),
    (width, height): (usize, usize),
    allow_diagonal: bool,
) -> Vec<(usize, usize)> {
    let adj: Vec<(i32, i32)> = if allow_diagonal {
        vec![
            (y as i32 - 1, x as i32),
            (y as i32 + 1, x as i32),
            (y as i32, x as i32 - 1),
            (y as i32, x as i32 + 1),
            (y as i32 - 1, x as i32 - 1),
            (y as i32 + 1, x as i32 + 1),
            (y as i32 - 1, x as i32 + 1),
            (y as i32 + 1, x as i32 - 1),
        ]
    } else {
        vec![
            (y as i32 - 1, x as i32),
            (y as i32 + 1, x as i32),
            (y as i32, x as i32 - 1),
            (y as i32, x as i32 + 1),
        ]
    };
    adj.into_iter()
        .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < height as i32 && *x < width as i32)
        .map(|(y, x)| (y as usize, x as usize))
        .collect::<Vec<(usize, usize)>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chunks_even_split() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut chunks = chunks(&data, 3);
        assert_eq!(chunks.next().unwrap(), &[1, 2, 3]);
        assert_eq!(chunks.next().unwrap(), &[4, 5, 6]);
        assert_eq!(chunks.next().unwrap(), &[7, 8, 9]);
    }
    #[test]
    fn test_chunks_uneven_split() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut chunks = chunks(&data, 3);
        assert_eq!(chunks.next().unwrap(), &[1, 2, 3]);
        assert_eq!(chunks.next().unwrap(), &[4, 5, 6]);
        assert_eq!(chunks.next().unwrap(), &[7, 8, 9, 10]);
    }

    #[test]
    fn test_get_lines_as_numbers() {
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(get_lines_as_numbers(input), expected);
    }
    #[test]
    fn test_get_lines_as_vec_char() {
        let input = "12\n23\n34\n45\n56\n67\n78\n89";
        let expected = vec![
            vec!['1', '2'],
            vec!['2', '3'],
            vec!['3', '4'],
            vec!['4', '5'],
            vec!['5', '6'],
            vec!['6', '7'],
            vec!['7', '8'],
            vec!['8', '9'],
        ];
        assert_eq!(get_lines_as_vec_chars(input), expected);
    }
    #[test]
    fn test_get_lines_as_vec_u32() {
        let input = "12\n23\n34\n45\n56\n67\n78\n89";
        let expected = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
            vec![7, 8],
            vec![8, 9],
        ];
        assert_eq!(get_lines_as_vec_u32(input), expected);
    }
    #[test]
    fn test_transpose() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let output = transpose(&input);
        let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(output, expected);
    }
}
