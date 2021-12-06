mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn execute() {
    let functions = [day1(), day2(), day3(), day4(), day5(), day6()];
    for (i, function) in functions.iter().enumerate() {
        for (part, f) in function.iter().enumerate() {
            println!("Day {} - Part {}: {}", i + 1, part + 1, f);
        }
    }
}

fn day1() -> Vec<String> {
    [day1::part1(), day1::part2()].to_vec()
}
fn day2() -> Vec<String> {
    [("Was replaced with part2.".into()), (day2::part1())].to_vec()
}
fn day3() -> Vec<String> {
    [day3::part1(), day3::part2()].to_vec()
}

fn day4() -> Vec<String> {
    let mut result = Vec::new();

    if let Some((field_sum, num, sum)) = day4::part1() {
        result.push(format!("{} * {} = {}", field_sum, num, sum));
    }
    if let Some((field_sum, num, sum)) = day4::part2() {
        result.push(format!("{} * {} = {}", field_sum, num, sum));
    }

    result
}

fn day5() -> Vec<String> {
    [
        format!("overlaps: {}", day5::part1()),
        format!("overlaps: {}", day5::part2()),
    ]
    .to_vec()
}

fn day6() -> Vec<String> {
    [
        format!("fishes: {}", day6::part1()),
        format!("fishes: {}", day6::part2()),
    ]
    .to_vec()
}
