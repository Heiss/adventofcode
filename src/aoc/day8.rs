use itertools::Itertools;
use std::collections::HashMap;

use crate::helper::read_in;

fn day8_part1(contents: &str) -> u32 {
    let mut result = 0;
    for line in contents.split("\n") {
        let mut split = line.split("|");
        let _left = split.next().expect("no left");
        let right = split.next().expect("no right");

        result += right
            .split_whitespace()
            .filter(|&v| match v.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count()
    }

    result as u32
}

fn day8_part2(contents: &str) -> u32 {
    let numbers = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    contents
        .split("\n")
        .map(|line| {
            let mut map: HashMap<char, char> = HashMap::new();

            let mut split = line.split("|");
            let left = split.next().expect("no left");
            let right = split.next().expect("no right");

            // find easy digits
            left.split_whitespace()
                .filter(|&v| match v.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .for_each(|v| {
                    let number = match v.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        7 => 8,
                        _ => panic!("not an easy digit"),
                    };

                    v.chars().sorted().enumerate().for_each(|(index, c)| {
                        let m = numbers[number].chars().nth(index).expect("char not found");
                        if let None = map.get(&m) {
                            map.insert(m, c);
                        }
                    })
                });

            let encode_numbers = numbers
                .iter()
                .map(|&v| {
                    v.chars()
                        .map(|c| map.get(&c).expect("char not in map"))
                        .sorted()
                        .collect::<String>()
                })
                .collect::<Vec<String>>();

            right
                .split_whitespace()
                .map(|v| {
                    let v: String = v.chars().sorted().collect();
                    encode_numbers
                        .iter()
                        .position(|n| *n == v)
                        .expect(&format!(
                            "encode not found, encode: {:?}, map: {:?}, value: {}",
                            encode_numbers, map, v
                        ))
                        .to_string()
                })
                .collect::<String>()
                .parse::<u32>()
                .expect("cannot be parsed into integer")
        })
        .sum()
}

pub fn part1() -> String {
    let contents = read_in("inputs/day8.txt");
    format!("count: {}", day8_part1(&contents))
}

pub fn part2() -> String {
    let contents = read_in("inputs/day8.txt");
    format!("count: {}", day8_part2(&contents))
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part1() {
        let cont =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(day8_part1(cont), 26);
    }

    #[test]
    fn test_part2() {
        let cont =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(day8_part2(cont), 61229);
    }

    #[test]
    fn test_part2_short() {
        let cont =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

        assert_eq!(day8_part2(cont), 5353);
    }
}
