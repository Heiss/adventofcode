use itertools::Itertools;

use crate::helper::read_in;

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn find_lowpoints(matrix: &Vec<Vec<u32>>) -> Vec<u32> {
    let line_pits: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.windows(3).enumerate().map(move |(column, val)| {
                if val[0] < val[1] {
                    Some((row, column))
                } else if val[0] > val[1] && val[1] < val[2] {
                    Some((row, column + 1))
                } else if val[1] > val[2] {
                    Some((row, column + 2))
                } else {
                    None
                }
            })
        })
        .flatten()
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .sorted()
        .dedup()
        .collect();

    let column_pits: Vec<(usize, usize)> = transpose(matrix)
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.windows(3).enumerate().map(move |(column, val)| {
                if val[0] < val[1] {
                    Some((row, column))
                } else if val[0] > val[1] && val[1] < val[2] {
                    Some((row, column + 1))
                } else if val[1] > val[2] {
                    Some((row, column + 2))
                } else {
                    None
                }
            })
        })
        .flatten()
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .sorted()
        .dedup()
        .collect();

    println!("{:?}\n{:?}", line_pits, column_pits);

    line_pits
        .iter()
        .filter(|(x, y)| column_pits.iter().find(|(m, n)| m == y && n == x).is_some())
        .map(|(x, y)| matrix[*x][*y].clone())
        .collect()
}

fn day9_part1(contents: &str) -> u32 {
    let matrix: Vec<Vec<u32>> = contents
        .split("\n")
        .map(|v| {
            v.chars()
                .map(|x| x.to_string().parse().expect(&format!("not a number {}", v)))
                .collect()
        })
        .collect();

    println!("{:?}\n{:?}", matrix, find_lowpoints(&matrix));

    find_lowpoints(&matrix).iter().sum()
}

fn day9_part2(contents: &str) -> u32 {
    0
}

pub fn part1() -> String {
    let contents = read_in("inputs/day8.txt");
    format!("sum: {}", day9_part1(&contents))
}

pub fn part2() -> String {
    let contents = read_in("inputs/day8.txt");
    format!("count: {}", day9_part2(&contents))
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(day9_part1(contents), 15)
    }
}
