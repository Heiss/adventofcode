mod day4;
mod day5;

pub fn execute() {
    let functions = [day4()];
    for (i, function) in functions.iter().enumerate() {
        for (part, f) in function.iter().enumerate() {
            println!("Day {} - Part {}: {}", i + 1, part + 1, f);
        }
    }
}

pub fn day4() -> Vec<String> {
    let mut result = Vec::new();

    if let Some((field_sum, num, sum)) = day4::part1() {
        result.push(format!("{} * {} = {}", field_sum, num, sum));
    }
    if let Some((field_sum, num, sum)) = day4::part2() {
        result.push(format!("{} * {} = {}", field_sum, num, sum));
    }

    result
}

pub fn day5() {
    day5::part1();
    day5::part2();
}
