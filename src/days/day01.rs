use crate::{Solution, SolutionPair};
use std::num::ParseIntError;
use std::str::FromStr;

/// Given a list of calories, with each elf seperated by \n\n,
/// return a list of the total calories for each elf
fn get_calorie_list(input: String) -> Vec<u64> {
    return input.trim().split("\n\n").map(total_calories).collect();
}

/// Given a list of calories, seperated by \n, return the total number of calories
fn total_calories(calories: &str) -> u64 {
    let calories_list = calories
        .split("\n")
        .map(u64::from_str)
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .expect("Failed to parse input");

    let calories_sum = calories_list.into_iter().sum();

    return calories_sum;
}

pub fn solve(input_data: String, input_star_data: String) -> SolutionPair {
    let calories_per_elf = get_calorie_list(input_data);
    let mut calories_per_elf_star = get_calorie_list(input_star_data);

    let max_elf_calories = calories_per_elf
        .into_iter()
        .max()
        .expect("Failed to find max");

    calories_per_elf_star.sort_by_key(|&c| std::cmp::Reverse(c));
    let top_three_calories_sum = calories_per_elf_star.into_iter().take(3).sum();

    (
        Solution::U64(max_elf_calories),
        Solution::U64(top_three_calories_sum),
    )
}
