use std::collections::HashMap;

use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::helper::read_in;

const NEEDS_DAYS: u8 = 7;
const FIRST_CYCLE: u8 = 9;

#[derive(Debug, Clone, Copy)]
struct Lanternfish {
    until_birth: u8,
}

impl Lanternfish {
    pub fn new(days: u8) -> Self {
        Lanternfish { until_birth: days }
    }

    pub fn tick(&mut self) -> Option<Self> {
        let mut results = None;

        if self.until_birth == 0 {
            results = Some(Lanternfish::default());
            self.until_birth = NEEDS_DAYS;
        }

        self.until_birth -= 1;

        results
    }
}

impl Default for Lanternfish {
    fn default() -> Self {
        Lanternfish {
            until_birth: FIRST_CYCLE - 1,
        }
    }
}

/// This is a paralleled implementation.
fn day6_part1(contents: &str, days: u32) -> usize {
    let mut fishes: Vec<Lanternfish> = contents
        .split(",")
        .map(|v| v.parse::<u8>().expect("not a number"))
        .map(|v| Lanternfish::new(v))
        .collect();

    for _ in 0..days {
        let temp = fishes
            .par_iter_mut()
            .filter_map(|fish| fish.tick())
            .collect::<Vec<Lanternfish>>();
        fishes.extend(temp);
    }

    fishes.len()
}

/// This is a faster implementation.
fn day6_part2(contents: &str, days: u32) -> usize {
    let mut fishes: HashMap<u8, u64> = HashMap::new();

    contents
        .split(",")
        .map(|v| v.parse::<u8>().expect("not a number"))
        .for_each(|v| *fishes.entry(v).or_insert(0) += 1);

    for _ in 0..days {
        let birth = *fishes.get(&0).unwrap_or(&0);

        for i in 1..FIRST_CYCLE {
            fishes.insert(i - 1, *fishes.get(&i).unwrap_or(&0));
        }

        *fishes.entry(NEEDS_DAYS - 1).or_insert(0) += birth;
        fishes.insert(FIRST_CYCLE - 1, birth);
    }

    fishes.iter().map(|(_, v)| *v as usize).sum()
}

pub fn part1() -> String {
    day6_part1(&read_in("inputs/day6.txt"), 80).to_string()
}

pub fn part2() -> String {
    day6_part2(&read_in("inputs/day6.txt"), 256).to_string()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "3,4,3,1,2";
        assert_eq!(day6_part1(contents, 18), 26);
        assert_eq!(day6_part1(contents, 80), 5934);
    }

    #[test]
    fn test_part2() {
        let contents = "3,4,3,1,2";
        assert_eq!(day6_part2(contents, 18), 26);
        assert_eq!(day6_part2(contents, 80), 5934);
        assert_eq!(day6_part2(contents, 256), 26984457539);
    }
}
