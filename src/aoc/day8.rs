use crate::helper::read_in;

fn day8_part1(contents: &str) -> u32 {
    let mut split = contents.split("|");
    let _left = split.next().expect("no left");
    let right = split.next().expect("no right");

    right
        .split_whitespace()
        .filter(|&v| match v.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count() as u32
}

pub fn part1() -> String {
    let contents = read_in("inputs/day8.txt");
    format!("count: {}", day8_part1(&contents))
}

pub fn part2() -> String {
    format!("count: {}", 0)
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
    fn test_part2() {}
}
