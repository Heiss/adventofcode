use std::{fs::File, io::Read, panic};

pub fn part1() -> String {
    let mut f = File::open("inputs/day3.txt").expect("No input.txt found");
    let mut buf: String = "".into();
    f.read_to_string(&mut buf).expect("Cannot read input.txt");

    let mut common = [0; 12];

    buf.split("\n").for_each(|v| {
        v.to_string()
            .chars()
            .enumerate()
            .for_each(|(i, c)| match c {
                '1' => common[i] += 1,
                '0' => common[i] -= 1,
                _ => panic!("invalid input"),
            });
    });

    let most: Vec<u8> = common
        .iter()
        .map(|&v| {
            if v > 0 {
                1
            } else if v < 0 {
                0
            } else {
                panic!("invalid most {}", v)
            }
        })
        .collect();

    let least: Vec<u8> = most
        .iter()
        .map(|&v| match v {
            1 => 0,
            0 => 1,
            _ => panic!("invalid least {}", v),
        })
        .collect();

    let num1 = to_u32(&most);
    let num2 = to_u32(&least);

    format!("{} * {} = {}", num1, num2, num1 * num2)
}

fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

pub fn part2() -> String {
    let mut f = File::open("inputs/day3.txt").expect("No input.txt found");
    let mut buf: String = "".into();
    f.read_to_string(&mut buf).expect("Cannot read input.txt");

    let numbers: Vec<&str> = buf.split("\n").collect();
    let mut temp_numbers = numbers.clone();

    let mut temp_index = 0;
    while temp_numbers.len() > 1 {
        let mut common = [0; 12];
        temp_numbers.iter().for_each(|&v| {
            v.to_string()
                .chars()
                .enumerate()
                .for_each(|(i, c)| match c {
                    '1' => common[i] += 1,
                    '0' => common[i] -= 1,
                    _ => panic!("invalid input"),
                });
        });

        let common_value_in_place = if common[temp_index] >= 0 {
            '0'
        } else if common[temp_index] < 0 {
            '1'
        } else {
            panic!("invalid common {}", common[temp_index])
        };

        temp_numbers = temp_numbers
            .iter()
            .filter(|&&v| {
                v.to_string()
                    .chars()
                    .skip(temp_index)
                    .next()
                    .expect("no valid number found")
                    == common_value_in_place
            })
            .map(|&v| v)
            .collect();

        temp_index += 1;
    }

    let most = temp_numbers[0];

    let mut temp_numbers = numbers.clone();

    let mut temp_index = 0;
    while temp_numbers.len() > 1 {
        let mut common = [0; 12];
        temp_numbers.iter().for_each(|&v| {
            v.to_string()
                .chars()
                .enumerate()
                .for_each(|(i, c)| match c {
                    '1' => common[i] += 1,
                    '0' => common[i] -= 1,
                    _ => panic!("invalid input"),
                });
        });

        let common_value_in_place = if common[temp_index] >= 0 {
            '1'
        } else if common[temp_index] < 0 {
            '0'
        } else {
            panic!("invalid common {}", common[temp_index])
        };

        temp_numbers = temp_numbers
            .iter()
            .filter(|&&v| {
                v.to_string()
                    .chars()
                    .skip(temp_index)
                    .next()
                    .expect("no valid number found")
                    == common_value_in_place
            })
            .map(|&v| v)
            .collect();

        temp_index += 1;
    }
    let least = temp_numbers[0];

    let num1 = to_u32(
        most.to_string()
            .chars()
            .map(|v| match v {
                '0' => 0,
                '1' => 1,
                _ => panic!("error"),
            })
            .collect::<Vec<u8>>()
            .as_slice(),
    );
    let num2 = to_u32(
        least
            .to_string()
            .chars()
            .map(|v| match v {
                '0' => 0,
                '1' => 1,
                _ => panic!("error"),
            })
            .collect::<Vec<u8>>()
            .as_slice(),
    );

    format!("{} * {} = {}", num1, num2, num1 * num2)
}
