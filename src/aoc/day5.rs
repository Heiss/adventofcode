use crate::helper::read_in;
use regex::Regex;
use std::mem;

const SIZE: usize = 1000;

#[warn(dead_code)]
fn day5_part1(contents: &str) -> String {
    let re = Regex::new(r"(?P<x1>.*),(?P<y1>.*) -> (?P<x2>.*),(?P<y2>.*)").unwrap();
    let mut arr = [[0u8; SIZE]; SIZE];

    for caps in re.captures_iter(&contents) {
        if caps["x1"] != caps["x2"] && caps["y1"] != caps["y2"] {
            continue;
        }

        let mut x1 = caps["x1"]
            .parse::<usize>()
            .expect(&format!("not a number {}", &caps["x1"]));
        let mut x2 = caps["x2"]
            .parse::<usize>()
            .expect(&format!("not a number {}", &caps["x2"]));
        let mut y1 = caps["y1"]
            .parse::<usize>()
            .expect(&format!("not a number {}", &caps["y1"]));
        let mut y2 = caps["y2"]
            .parse::<usize>()
            .expect(&format!("not a number {}", &caps["y2"]));

        if x1 > x2 {
            mem::swap(&mut x1, &mut x2);
        }

        if y1 > y2 {
            mem::swap(&mut y1, &mut y2);
        }

        for i in x1..=x2 {
            for j in y1..=y2 {
                arr[j][i] += 1;
            }
        }
    }

    arr.iter()
        .flatten()
        .filter(|&&v| v >= 2)
        .count()
        .to_string()
}

fn day5(contents: &str, diagonal: bool) -> String {
    let re = Regex::new(r"(?P<x1>.*),(?P<y1>.*) -> (?P<x2>.*),(?P<y2>.*)").unwrap();
    let mut arr = [[0u8; SIZE]; SIZE];

    for caps in re.captures_iter(&contents) {
        let x1 = caps["x1"]
            .parse::<isize>()
            .expect(&format!("not a number {}", &caps["x1"]));
        let x2 = caps["x2"]
            .parse::<isize>()
            .expect(&format!("not a number {}", &caps["x2"]));
        let y1 = caps["y1"]
            .parse::<isize>()
            .expect(&format!("not a number {}", &caps["y1"]));
        let y2 = caps["y2"]
            .parse::<isize>()
            .expect(&format!("not a number {}", &caps["y2"]));

        let mut dy = y2 - y1;
        let mut dx = x2 - x1;

        if !diagonal && (dx != 0 && dy != 0) {
            println!("skip");
            continue;
        }

        if dy != 0 {
            dy = dy.signum() * (dy / dy);
        }

        if dx != 0 {
            dx = dx.signum() * (dx / dx);
        }

        let mut temp_x = x1;
        let mut temp_y = y1;

        let mut points: Vec<(usize, usize)> = Vec::new();
        points.push((x2 as usize, y2 as usize));

        println!("x {} y {}", dx, dy);

        while temp_x != x2 || temp_y != y2 {
            points.push((temp_x as usize, temp_y as usize));

            temp_x += dx;
            temp_y += dy;
        }

        println!("{:?}", points);
        for (i, j) in points {
            println!("{} {}", i, j);
            arr[i][j] += 1;
        }
    }

    arr.iter()
        .flatten()
        .filter(|&&v| v >= 2)
        .count()
        .to_string()
}

pub fn part1() -> String {
    let contents = read_in("inputs/day5.txt");
    day5(&contents, false)
}

pub fn part2() -> String {
    let contents = read_in("inputs/day5.txt");
    day5(&contents, true)
}

#[test]
fn test_part1() {
    let input = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";
    assert_eq!(day5(input, false), "5");
}

#[test]
fn test_part2() {
    let input = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";
    assert_eq!(day5(input, true), "12");
}
