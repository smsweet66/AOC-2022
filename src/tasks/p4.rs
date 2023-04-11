use regex::Regex;
use crate::tasks::helper::get_lines;

/*
 * Reads a file that contains a list of pairs of tasks.
 * Each pair is two range of task ids, separated by a comma.
 * This function returns the number of pairs where one range of tasks
 * fully contains the other range.
 */
pub fn count_contained_tasks(filename: &str) -> u32
{
    let lines = get_lines(filename);
    let pattern = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    return lines.iter()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            let range_1: (u32, u32) = (captures[1].parse().unwrap(), captures[2].parse().unwrap());
            let range_2: (u32, u32) = (captures[3].parse().unwrap(), captures[4].parse().unwrap());

            return if range_1.0 <= range_2.0 && range_1.1 >= range_2.1 { 1 }
            else if range_2.0 <= range_1.0 && range_2.1 >= range_1.1 { 1 }
            else { 0 }
        })
        .sum();
}

/*
 * Reads a file that contains a list of pairs of tasks.
 * Each pair is two range of task ids, separated by a comma.
 * This function returns the number of pairs where one range of tasks
 * overlaps the other range at all.
 */
pub fn count_overlapping_tasks(filename: &str) -> u32
{
    let lines = get_lines(filename);
    let pattern = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    return lines.iter()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            let range_1: (u32, u32) = (captures[1].parse().unwrap(), captures[2].parse().unwrap());
            let range_2: (u32, u32) = (captures[3].parse().unwrap(), captures[4].parse().unwrap());

            return if range_1.0 <= range_2.0 && range_1.1 >= range_2.0 { 1 }
            else if range_2.0 <= range_1.0 && range_2.1 >= range_1.0 { 1 }
            else { 0 }
        })
        .sum();
}