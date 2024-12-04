use crate::aoc::day1;
use crate::aoc::day2;
use crate::aoc::helpers;
use anyhow::{self, Ok};
pub mod aoc;

fn main() -> anyhow::Result<()> {
    let inputs = helpers::open_inputs("inputs/day1_part1.txt")?;

    let (first_list, second_list) = day1::str_to_lists(&inputs)?;

    let total = day1::sum_lists(first_list.clone(), second_list.clone());

    println!("day1 part A: result: {}", total);
    let score = day1::similarity_score(first_list, second_list)?;
    println!("day1 part B: similarity score: {score}");

    let safe_reports_a = day2::part_a("inputs/day2.txt")?;
    println!("day2 part A: safe reports: {safe_reports_a}");

    let safe_reports_b = day2::part_b("inputs/day2.txt")?;
    println!("day2 part B: safe reports with dampener: {safe_reports_b}");
  
    Ok(())
}
