use crate::helper::read_in;
use regex::Regex;

const SIZE: usize = 1000;

pub fn part1() -> String {
    let contents = read_in("inputs/day5.txt");
    let re = Regex::new(r"(?P<x1>.*),(?P<y1>.*) -> (?P<x2>.*),(?P<y2>.*)").unwrap();
    let mut arr = [[0u8; SIZE]; SIZE];

    for caps in re.captures_iter(&contents) {
        if &caps["x1"] != &caps["x2"] && &caps["y1"] != &caps["y2"] {
            continue;
        }

        let x1 = caps["x1"]
            .parse::<usize>()
            .expect(&format!("not a number {}", &caps["x1"]));
        let x2 = caps["x2"]
            .parse::<usize>()
            .expect(&format!("not a number {}", &caps["x2"]));
        let y1 = caps["y1"]
            .parse::<usize>()
            .expect(&format!("not a number {}", &caps["y1"]));
        let y2 = caps["y2"]
            .parse::<usize>()
            .expect(&format!("not a number {}", &caps["y2"]));

        for i in x1..=x2 {
            for j in y1..=y2 {
                arr[i][j] += 1;
            }
        }
    }

    arr.iter().flatten().filter(|&&v| v > 1).count().to_string()
}

pub fn part2() -> String {
    String::new()
}
