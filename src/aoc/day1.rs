use std::{fs::File, io::Read};

pub fn part1() -> String {
    let mut f = File::open("inputs/day1.txt").expect("No input.txt found");

    let mut buf: String = "".into();
    f.read_to_string(&mut buf).expect("Cannot read input.txt");

    let mut iter = buf
        .split("\n")
        .map(|v| v.parse().expect("Cannot parse string into number"))
        .collect::<Vec<i32>>()
        .into_iter();

    let mut count = 0;
    let mut prev = iter.next().expect("Cannot extract first element");

    for curr in iter {
        if prev < curr {
            count += 1;
        }

        prev = curr;
    }

    format!(
        "{} measurements are larger than the previous measurement",
        count
    )
}

pub fn part2() -> String {
    let mut f = File::open("inputs/day1.txt").expect("No input.txt found");
    let mut buf: String = "".into();
    f.read_to_string(&mut buf).expect("Cannot read input.txt");

    let iter = buf
        .split("\n")
        .map(|v| v.parse().expect("Cannot parse string into number"))
        .collect::<Vec<i32>>()
        .into_iter();

    let vec = iter
        .as_slice()
        .windows(3)
        .map(|u| u.iter().sum())
        .collect::<Vec<i32>>();

    let count: i32 = vec
        .windows(2)
        .map(|v| match v[0] < v[1] {
            true => 1,
            false => 0,
        })
        .sum();

    format!(
        "{} measurements are larger than the previous measurement",
        count
    )
}
