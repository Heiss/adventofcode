use crate::helper::read_in;

fn median(list: &Vec<u32>) -> u32 {
    let mut sortedlist = list.clone();

    sortedlist.sort();

    let len = sortedlist.len();

    if len % 2 == 0 {
        let n1 = *sortedlist.get((len) / 2).expect("no median");
        let n2 = *sortedlist.get((len + 1) / 2).expect("no median");

        (n1 + n2) / 2
    } else {
        *sortedlist.get((len + 1) / 2).expect("no median")
    }
}

fn termial(max: u32) -> u32 {
    let mut res = 0;
    for i in 1..=max {
        res += i;
    }
    res
}

fn day7_part1(contents: &str) -> u32 {
    let crabs = contents
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let median = median(&crabs);

    crabs.iter().map(|&v| median.max(v) - median.min(v)).sum()
}

fn day7_part2(contents: &str) -> u32 {
    let crabs = contents
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let avg: u32 = crabs.iter().sum::<u32>() / crabs.len() as u32;

    crabs
        .iter()
        .map(|&v| avg.max(v) - avg.min(v))
        .map(termial)
        .sum()
}

pub fn part1() -> String {
    let contents = read_in("inputs/day7.txt");
    format!("fuel: {}", day7_part1(&contents))
}

pub fn part2() -> String {
    let contents = read_in("inputs/day7.txt");
    format!("fuel: {}", day7_part2(&contents))
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part1() {
        let cont = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(day7_part1(cont), 37);
    }

    #[test]
    fn test_part2() {
        let cont = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(day7_part2(cont), 168);
    }

    #[test]
    fn test_termial() {
        assert_eq!(termial(0), 0);
        assert_eq!(termial(1), 1);
        assert_eq!(termial(2), 3);
        assert_eq!(termial(3), 6);
        assert_eq!(termial(4), 10);
    }
}
