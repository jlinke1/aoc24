use crate::aoc::day1;
use crate::aoc::helpers;
use anyhow::{self, Ok};
pub mod aoc;

fn main() -> anyhow::Result<()> {
    let inputs = helpers::open_inputs("inputs/day1_part1.txt")?;

    let (first_list, second_list) = day1::str_to_lists(&inputs)?;

    let total = day1::sum_lists(first_list.clone(), second_list.clone());

    println!("result: {}", total);
    let score = day1::similarity_score(first_list, second_list)?;
    println!("similarity score: {score}");
    Ok(())
}
