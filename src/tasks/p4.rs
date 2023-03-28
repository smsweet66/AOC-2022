/*
 * Reads a file that contains a list of pairs of tasks.
 * Each pair is two range of task ids, separated by a comma.
 * This function returns the number of pairs where one range of tasks
 * fully contains the other range.
 */
use crate::tasks::helper::get_lines;

pub fn count_contained_tasks(filename: &str) -> u32
{
    let lines = get_lines(filename);
    let mut count = 0;
    for line in lines
    {
        let ranges: Vec<&str> = line.split(',').collect();
        let range_1_str: Vec<&str> = ranges[0].split('-').collect();
        let range_2_str: Vec<&str> = ranges[1].split('-').collect();
        let range_1: (u32, u32) = (range_1_str[0].parse().unwrap(), range_1_str[1].parse().unwrap());
        let range_2: (u32, u32) = (range_2_str[0].parse().unwrap(), range_2_str[1].parse().unwrap());
        if range_1.0 <= range_2.0 && range_1.1 >= range_2.1
        { count += 1; }
        else if range_2.0 <= range_1.0 && range_2.1 >= range_1.1
        { count += 1; }
    }

    return count;
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
    let mut count = 0;
    for line in lines
    {
        let ranges: Vec<&str> = line.split(',').collect();
        let range_1_str: Vec<&str> = ranges[0].split('-').collect();
        let range_2_str: Vec<&str> = ranges[1].split('-').collect();
        let range_1: (u32, u32) = (range_1_str[0].parse().unwrap(), range_1_str[1].parse().unwrap());
        let range_2: (u32, u32) = (range_2_str[0].parse().unwrap(), range_2_str[1].parse().unwrap());
        if range_1.0 <= range_2.0 && range_1.1 >= range_2.0
        { count += 1; }
        else if range_2.0 <= range_1.0 && range_2.1 >= range_1.0
        { count += 1; }
    }

    return count;
}