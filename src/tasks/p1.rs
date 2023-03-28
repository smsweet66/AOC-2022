use std::cmp::Reverse;
use std::collections::BinaryHeap;
use crate::tasks::helper::get_lines;

/*
 * Reads input from a file containing a list of calorie values held by the elves.
 * Each elf's list of items contains a blank space in between
 * Calculates the greatest group of calories from the list and returns it
 */
pub fn get_greatest_calories(input_file: &str) -> u32
{
    let lines = get_lines(input_file);

    let mut greatest_calories: u32 = 0;
    let mut current_calories: u32 = 0;
    for line in lines
    {
        if line == ""
        {
            if current_calories > greatest_calories
            { greatest_calories = current_calories }

            current_calories = 0;
        }
        else { current_calories += line.parse::<u32>().unwrap() }
    }

    return greatest_calories;
}

/*
 * Reads input from a file containing a list of calorie values held by the elves.
 * Each elf's list of items contains a blank space in between
 * Calculates the greatest three groups of calories from the list and returns it
 */
pub fn get_greatest_three_calories(input_file: &str) -> u32
{
    let lines = get_lines(input_file);

    let mut calories: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let mut current_calories: u32 = 0;
    for line in lines
    {
        if line == ""
        {
            calories.push(Reverse(current_calories));
            current_calories = 0;
            if calories.len() > 3
            { calories.pop(); }
        }
        else { current_calories += line.parse::<u32>().unwrap(); }
    }

    let mut greatest_three_sum: u32 = 0;
    while calories.len() > 0
    { greatest_three_sum += calories.pop().unwrap().0; }

    return greatest_three_sum;
}