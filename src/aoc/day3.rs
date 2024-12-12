use std::i32;

use regex::Regex;

use super::helpers;

pub fn part_a(path: &str) -> anyhow::Result<i32> {
    let inputs = helpers::open_inputs(path)?;

    compute(&inputs)
}

pub fn part_b(path: &str) -> anyhow::Result<i32> {
    let inputs = helpers::open_inputs(path)?;

    split_and_compute(&inputs)
}

fn split_and_compute(haystack: &str) -> anyhow::Result<i32> {
    let parts: Vec<&str> = haystack.splitn(2, "don't()").collect();
    if parts.len() < 2 {
        return compute(haystack);
    }

    let new_haystack = parts[1].splitn(2, "do()").collect::<Vec<_>>()[1];

    Ok(compute(parts[0])? + split_and_compute(new_haystack)?)
}

fn compute(haystack: &str) -> anyhow::Result<i32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut result = 0;
    for (_, [first, second]) in re.captures_iter(haystack).map(|c| c.extract()) {
        let first_num: i32 = first.parse()?;
        let second_num: i32 = second.parse()?;
        result += first_num * second_num;
    }

    Ok(result)
}
