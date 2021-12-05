use std::fmt;

use crate::helper::read_in;

#[derive(Debug)]
struct Field {
    arr: Vec<Vec<String>>, // arr[row][col]
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "---\n{}\n---",
            self.arr
                .iter()
                .map(|v| v.as_slice().join(" "))
                .collect::<Vec<String>>()
                .as_slice()
                .join("\n")
        )
    }
}

impl Field {
    pub fn new(str_arr: &str) -> Self {
        Field {
            arr: str_arr
                .split("\n")
                .map(|line| {
                    line.to_string()
                        .split_whitespace()
                        .map(|v| v.to_string())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn get_sum(&self) -> u32 {
        self.arr
            .iter()
            .flatten()
            .map(|v| v.parse::<u32>().unwrap_or(0))
            .sum()
    }

    fn bingo(&self) -> bool {
        let n = self.arr.len();
        for i in 0..n {
            if self.bingo_row(i) || self.bingo_col(i) {
                return true;
            }
        }

        return false;
    }

    fn bingo_row(&self, row: usize) -> bool {
        let n = self.arr.len();
        for i in 0..n {
            if self.arr[row][i] != "x" {
                return false;
            }
        }

        true
    }

    fn bingo_col(&self, col: usize) -> bool {
        let n = self.arr.len();
        for i in 0..n {
            if self.arr[i][col] != "x" {
                return false;
            }
        }

        true
    }

    fn check(&mut self, number: &str) {
        for i in self.arr.iter_mut().flatten() {
            if i == number {
                *i = "x".into();
            }
        }
    }

    pub fn check_number(&mut self, number: &str) -> Option<(u32, u32, u32)> {
        self.check(number);

        if self.bingo() {
            let number = number.parse::<u32>().expect("not a number given");
            let sum = self.get_sum();
            Some((sum, number, sum * number))
        } else {
            None
        }
    }
}

pub fn part1() {
    let content = read_in("inputs/day4.txt");
    let mut iter = content.split("\n\n");
    let first_line = iter.next().expect("no first line");

    let mut bingo_fields: Vec<Field> = Vec::new();

    while let Some(bingo) = iter.next() {
        bingo_fields.push(Field::new(bingo));
    }

    let bingo_run = first_line
        .split(",")
        .find_map(|v| bingo_fields.iter_mut().find_map(|b| b.check_number(v)));

    if let Some((field_sum, num, sum)) = bingo_run {
        println!("Day4 - Part1: {} * {} = {}", field_sum, num, sum);
    }
}

pub fn part2() {
    let content = read_in("inputs/day4.txt");
    let mut iter = content.split("\n\n");
    let first_line = iter.next().expect("no first line");

    let mut bingo_fields: Vec<Field> = Vec::new();

    while let Some(bingo) = iter.next() {
        bingo_fields.push(Field::new(bingo));
    }

    let mut latest_field = None;

    for number in first_line.split(",") {
        for b in &mut bingo_fields {
            b.check(number);
        }

        let current_bingos: Vec<&Field> =
            bingo_fields.iter().filter(|&field| field.bingo()).collect();

        let mut current_sum = None;
        for field in current_bingos {
            let num = field.get_sum();
            current_sum = match current_sum {
                Some(v) => {
                    if v > num {
                        Some(num)
                    } else {
                        Some(v)
                    }
                }
                None => Some(num),
            }
        }

        if let Some(v) = current_sum {
            let num: u32 = number.parse().unwrap();
            latest_field = Some((v, num, v * num));
        }

        bingo_fields = bingo_fields
            .into_iter()
            .filter(|field| !field.bingo())
            .collect();
    }

    let mut result = "Not working.".to_string();

    if let Some((field_sum, num, sum)) = latest_field {
        result = format!("{} * {} = {}", field_sum, num, sum);
    }

    println!("Day4 - Part2: {}", result);
}
