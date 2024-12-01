use std::i32;

use anyhow;

pub fn str_to_lists(input: &str) -> anyhow::Result<(Vec<i32>, Vec<i32>)> {
    let rows = input.split('\n');
    let mut first: Vec<i32> = vec![];
    let mut last: Vec<i32> = vec![];

    for row in rows {
        let parts: Vec<&str> = row.split("   ").collect();
        if parts.len() < 2 {
            break;
        }

        let first_num: i32 = parts[0].parse()?;
        let last_num: i32 = parts[1].parse()?;

        first.push(first_num);
        last.push(last_num);
    }
    first.sort();
    last.sort();

    Ok((first, last))
}

pub fn sum_lists(first: Vec<i32>, second: Vec<i32>) -> i32 {
    let mut result = 0;
    for (a, b) in first.into_iter().zip(second.into_iter()) {
        result += (a - b).abs();
    }

    result
}

pub fn similarity_score(left: Vec<i32>, right: Vec<i32>) -> anyhow::Result<i32> {
    let mut score = 0;

    for i in left {
        let count_of_i: i32 = right.iter().filter(|&&x| x == i).count().try_into()?;
        score += i * count_of_i
    }

    Ok(score)
}
