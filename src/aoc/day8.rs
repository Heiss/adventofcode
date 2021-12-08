use std::collections::HashMap;

use itertools::Itertools;

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
    contents
        .split("\n")
        .map(|line| {
            let mut map: HashMap<usize, String> = HashMap::new();

            let mut split = line.split("|");
            let left = split.next().expect("no left");
            let right = split.next().expect("no right");

            // find easy digits
            left.split_whitespace()
                .filter(|&word| match word.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .for_each(|word| {
                    let number = match word.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        7 => 8,
                        _ => panic!("not an easy digit"),
                    };

                    map.insert(number, word.to_string());
                });

            // find 6
            if let Some(six) = left
                .split_whitespace()
                .filter(|&word| {
                    word.len() == 6
                        && word
                            .chars()
                            .any(|c| map.get(&1).expect("no 1 found").chars().all(|f| f != c))
                })
                .next()
            {
                map.insert(6, six.to_string());
            }

            // find 0
            if let Some(zero) = left
                .split_whitespace()
                .filter(|&word| {
                    word.len() == 6
                        && word
                            .chars()
                            .any(|c| !map.get(&4).expect("no 4 found").chars().all(|f| f != c))
                        && !map.values().any(|f| f == word)
                })
                .next()
            {
                map.insert(0, zero.to_string());
            }

            // find 9
            if let Some(nine) = left
                .split_whitespace()
                .filter(|&word| word.len() == 6 && !map.values().any(|f| f == word))
                .next()
            {
                map.insert(9, nine.to_string());
            }

            // find 5
            if let Some(five) = left
                .split_whitespace()
                .filter(|&word| {
                    word.len() == 5
                        && word
                            .chars()
                            .all(|c| map.get(&6).expect("no 6 found").chars().any(|f| f == c))
                })
                .next()
            {
                map.insert(5, five.to_string());
            }

            // find 3
            if let Some(three) = left
                .split_whitespace()
                .filter(|&word| {
                    word.len() == 5
                        && word
                            .chars()
                            .all(|c| map.get(&9).expect("no 9 found").chars().any(|f| f == c))
                        && !map.values().any(|f| f == word)
                })
                .next()
            {
                map.insert(3, three.to_string());
            }

            // find 2
            if let Some(two) = left
                .split_whitespace()
                .filter(|&word| word.len() == 5 && !map.values().any(|f| f == word))
                .next()
            {
                map.insert(2, two.to_string());
            }

            let s = right
                .split_whitespace()
                .map(|word| {
                    let (&num, _) = map
                        .iter()
                        .filter(|(_, val)| word.chars().all(|f| val.chars().any(|c| c == f)))
                        .next()
                        .expect(&format!("word not found: {}", word));
                    num.to_string()
                })
                .collect::<String>()
                .parse::<u32>()
                .expect("cannot be parsed into integer");
            println!("map: {:?}, res: {}", map, s);
            s
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
