use crate::tasks::helper;

/*
 * Reads from a file that contains the contents of a bunch of rucksacks.
 * Each rucksack has two compartments of equal size
 * For each rucksack, there is a duplicate item in each compartment.
 * This function returns the sum of priorities of all the duplicate items.
 */
pub fn sum_priorities(rucksack_file: &str) -> u32
{
    let lines = helper::get_lines(rucksack_file);

    return lines.iter()
        .map(|line| {
        let compartments = line.split_at(line.len() / 2);
        for item in compartments.0.chars()
        {
            if compartments.1.contains(item)
            {
                return if item.is_lowercase() { item as u32 - 96 }
                else { item as u32 - 38 }
            }
        }

        return 0;
    })
    .sum();
}

/*
 * Reads from a file that contains the contents of a bunch of rucksacks.
 * Every three lines corresponds to a group of elves.
 * Each elf has exactly one item in common with the other two elves.
 * This function returns the sum of priorities of all the duplicate items per group.
 */
pub fn sum_group_priorities(rucksack_file: &str) -> u32
{
    let lines = helper::get_lines(rucksack_file);
    let mut sum: u32 = 0;
    let mut group: [String; 3] = [String::new(), String::new(), String::new()];
    let mut group_index: usize = 0;
    for line in lines
    {
        group[group_index] = String::from(line);
        group_index += 1;
        if group_index == 3
        {
            for item in group[0].chars()
            {
                if group[1].contains(item) && group[2].contains(item)
                {
                    if item.is_lowercase()
                    { sum += item as u32 - 96; }
                    else
                    { sum += item as u32 - 38; }

                    break;
                }
            }

            group_index = 0;
        }
    }

    return sum;
}