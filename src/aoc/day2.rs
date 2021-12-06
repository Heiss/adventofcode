use std::{fs::File, io::Read};

#[derive(Debug)]
struct Submarine {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl Submarine {
    fn new() -> Self {
        Submarine {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }

    fn forward(&mut self, number: i32) {
        self.horizontal += number;
        self.depth += self.aim * number;
    }

    fn down(&mut self, number: i32) {
        self.aim += number;
    }
}

pub fn part1() -> String {
    let mut f = File::open("inputs/day2.txt").expect("No input.txt found");
    let mut buf: String = "".into();
    f.read_to_string(&mut buf).expect("Cannot read input.txt");

    let iter = buf
        .split("\n")
        .map(|v| {
            let mut split = v.split_whitespace();
            (
                split.next().unwrap(),
                split
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .expect("cannot parse second argument"),
            )
        })
        .collect::<Vec<(&str, i32)>>()
        .into_iter();

    let mut submarine = Submarine::new();

    for m in iter {
        match m {
            ("forward", v) => submarine.forward(v),
            ("up", v) => submarine.down(-v),
            (_, v) => submarine.down(v),
        }
    }

    format!("fixed position value: {}", submarine.depth * submarine.horizontal)
}
