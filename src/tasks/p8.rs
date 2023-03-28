use crate::tasks::helper::get_lines;

/*
 * this functions checks if a given character is greater than all characters in a string
 */
pub fn is_greater_than_all(number: char, array: &str) -> bool
{
    for i in 0..array.len()
    {
        if number <= array.chars().nth(i).unwrap()
        { return false; }
    }
    return true;
}

/*
 * this function returns the number of characters that are less than the given character
 * until a character greater than or equal to the given character is found
 */
pub fn get_number_of_less_than(number: char, array: &str) -> u32
{
    let mut number_of_less_than = 0;
    for i in 0..array.len()
    {
        if number > array.chars().nth(i).unwrap()
        { number_of_less_than += 1; }
        else
        {
            number_of_less_than += 1;
            break;
        }
    }

    return number_of_less_than;
}


/*
 * The input is a file containing a grid of numbers representing tree heights.
 * A tree is visible from the outside if all the trees between it and the outside
 * in any of the four cardinal directions are shorter than it.
 * This function returns the number of visible trees.
 */
pub fn count_visible_trees(filename: &str) -> u32
{
    let lines = get_lines(filename);
    let mut visible_trees = 0;
    for i in 0..lines.len()
    {
        for j in 0..lines[i].len()
        {
            let current = lines[i].chars().nth(j).unwrap();

            // check if the current tree is visible from the left
            if is_greater_than_all(current, &lines[i][0..j])
            {
                visible_trees += 1;
                continue;
            }

            // check if the current tree is visible from the right
            if is_greater_than_all(current, &lines[i][j+1..])
            {
                visible_trees += 1;
                continue;
            }

            // check if the current tree is visible from the top
            let mut top = String::new();
            for k in 0..i
            { top.push(lines[k].chars().nth(j).unwrap()); }

            if is_greater_than_all(current, &top)
            {
                visible_trees += 1;
                continue;
            }

            // check if the current tree is visible from the bottom
            let mut bottom = String::new();
            for k in i+1..lines.len()
            { bottom.push(lines[k].chars().nth(j).unwrap()); }

            if is_greater_than_all(current, &bottom)
            {
                visible_trees += 1;
                continue;
            }
        }
    }

    return visible_trees;
}

/*
 * Given a file containing a grid of numbers representing tree heights, this function returns
 * the greatest scenic score of all trees.  This is calculated by multiplying the number of visible
 * trees in all four directions from the tree.  The scenic score of a tree on the outside is 0,
 * since it has no trees in at least one direction.
 */
pub fn get_greatest_scenic_score(filename: &str) -> u32
{
    let lines = get_lines(filename);
    let mut greatest_scenic_score = 0;
    for i in 1..lines.len()-1
    {
        for j in 1..lines[i].len()-1
        {
            let current = lines[i].chars().nth(j).unwrap();
            let mut scenic_score = 1;

            // get number of trees visible from the left
            scenic_score *= get_number_of_less_than(current, &lines[i][0..j].chars().rev().collect::<String>());;

            // get number of trees visible from the right
            scenic_score *= get_number_of_less_than(current, &lines[i][j+1..]);

            // get number of trees visible from the top
            let mut top = String::new();
            for k in (0..i).rev()
            { top.push(lines[k].chars().nth(j).unwrap()); }

            scenic_score *= get_number_of_less_than(current, &top);

            // get number of trees visible from the bottom
            let mut bottom = String::new();
            for k in i+1..lines.len()
            { bottom.push(lines[k].chars().nth(j).unwrap()); }

            scenic_score *= get_number_of_less_than(current, &bottom);

            if scenic_score > greatest_scenic_score
            { greatest_scenic_score = scenic_score; }
        }
    }

    return greatest_scenic_score;
}