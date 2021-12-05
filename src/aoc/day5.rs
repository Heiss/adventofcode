use crate::helper::read_in;
use regex::Regex;

const SIZE: usize = 1000;

fn day5(contents: &str) -> String {
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

pub fn part1() -> String {
    let contents = read_in("inputs/day5.txt");
    day5(&contents)
}

pub fn part2() -> String {
    String::new()
}

#[test]
fn test_input() {
    let input = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";
    assert_eq!(day5(input), "5");
}
