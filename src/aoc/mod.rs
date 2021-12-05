mod day4;
mod day5;

pub fn execute() {
    let functions = [day4(), day5()];
    for (i, function) in functions.iter().enumerate() {
        for (part, f) in function.iter().enumerate() {
            println!("Day {} - Part {}: {}", i + 1, part + 1, f);
        }
    }
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
    let mut result = Vec::new();

    result.push(format!("overlaps: {}", day5::part1()));
    result.push(format!("overlaps: {}", day5::part2()));

    result
}
