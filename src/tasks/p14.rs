use std::cmp::{max, min};
use std::io::Lines;
use crate::tasks::helper::get_lines;

#[derive(Clone, PartialEq)]
enum Spot
{
    Sand,
    Rock,
    Empty,
}

/*
 * The input if a file containing several lines describing different rock formations
 * The lines are split up into x,y coordinates seperated by " -> ".
 * -> means that the rock forms a line from the first coordinate to the second point.
 * Each line will always be a horizontal or vertical line.
 * Sand falls one piece at a time from the point 500,0.  The sand will attempt to go
 * straight down, when it can't do that, it will attempt to go diagonally down and
 * to the left, then down and to the right.  If it cannot perform any of these
 * actions, it will stop, and the next piece of sand will fall.
 * This function will return the number of pieces of sand that will fall until the sand
 * is no longer bound by the rock formations.
 */
pub fn get_num_sand_pieces(filename: &str) -> i32
{
    let lines = get_lines(filename);

    let mut min_x = u32::MAX;
    let mut max_x = u32::MIN;
    let mut max_y = u32::MIN;
    for line in &lines
    {
        let coords = line.split(" -> ").collect::<Vec<&str>>();
        for coord in coords
        {
            let x_y = coord.split(",").collect::<Vec<&str>>();
            let x = x_y[0].parse::<u32>().unwrap();
            let y = x_y[1].parse::<u32>().unwrap();

            if x < min_x
            { min_x = x; }
            else if x > max_x
            { max_x = x; }

            if y > max_y
            { max_y = y; }
        }
    }

    let mut grid = vec![vec![Spot::Empty; (max_x - min_x + 1) as usize]; (max_y + 1) as usize];
    for line in &lines
    {
        let coords = line.split(" -> ").collect::<Vec<&str>>();
        for i in 0..coords.len() -1
        {
            let x_y = coords[i].split(",").collect::<Vec<&str>>();
            let x = x_y[0].parse::<u32>().unwrap();
            let y = x_y[1].parse::<u32>().unwrap();

            let x_y = coords[i+1].split(",").collect::<Vec<&str>>();
            let x2 = x_y[0].parse::<u32>().unwrap();
            let y2 = x_y[1].parse::<u32>().unwrap();

            if x == x2
            {
                if y < y2
                {
                    for y in y..=y2
                    { grid[y as usize][x as usize - min_x as usize] = Spot::Rock; }
                }
                else
                {
                    for y in y2..=y
                    { grid[y as usize][x as usize - min_x as usize] = Spot::Rock; }
                }
            }
            else if y == y2
            {
                if x < x2
                {
                    for x in x..=x2
                    { grid[y as usize][x as usize - min_x as usize] = Spot::Rock; }
                }
                else
                {
                    for x in x2..=x
                    { grid[y as usize][x as usize - min_x as usize] = Spot::Rock; }
                }
            }
        }
    }

    let mut num_sand_pieces = 0;
    'outer: loop
    {
        let mut x = 500 - min_x;
        let mut y = 0;
        loop
        {
            if y == max_y   // Sand has overflown
            { break 'outer; }

            if grid[(y + 1) as usize][x as usize] == Spot::Empty
            { y += 1; }
            else if x == 0
            { break 'outer; }
            else if grid[(y + 1) as usize][(x - 1) as usize] == Spot::Empty
            { y += 1; x -= 1; }
            else if x + 1 > max_x
            { break 'outer; }
            else if grid[(y + 1) as usize][(x + 1) as usize] == Spot::Empty
            { y += 1; x += 1; }
            else
            {
                grid[y as usize][x as usize] = Spot::Sand;
                num_sand_pieces += 1;
                break;
            }
        }
    }

    num_sand_pieces
}

/*
 * Same as the function above, but now there is a floor two levels below the lowest
 * rock formation.  The sand will flow until there is sand blocking 500,0.
 */
pub fn get_num_sand_pieces_floored(filename: &str) -> i32
{
    let lines = get_lines(filename);

    let mut min_x = u32::MAX;
    let mut max_x = u32::MIN;
    let mut max_y = u32::MIN;
    for line in &lines
    {
        let coords = line.split(" -> ").collect::<Vec<&str>>();
        for coord in coords
        {
            let x_y = coord.split(",").collect::<Vec<&str>>();
            let x = x_y[0].parse::<u32>().unwrap();
            let y = x_y[1].parse::<u32>().unwrap();

            if x < min_x
            { min_x = x; }
            else if x > max_x
            { max_x = x; }

            if y > max_y
            { max_y = y; }
        }
    }

    max_y += 2;
    min_x = min(min_x, 500 - max_y);
    max_x = max(max_x, 500 + max_y);

    let mut grid = vec![vec![Spot::Empty; (max_x - min_x + 1) as usize]; (max_y + 1) as usize];
    for line in &lines
    {
        let coords = line.split(" -> ").collect::<Vec<&str>>();
        for i in 0..coords.len() -1
        {
            let x_y = coords[i].split(",").collect::<Vec<&str>>();
            let x = x_y[0].parse::<u32>().unwrap();
            let y = x_y[1].parse::<u32>().unwrap();

            let x_y = coords[i+1].split(",").collect::<Vec<&str>>();
            let x2 = x_y[0].parse::<u32>().unwrap();
            let y2 = x_y[1].parse::<u32>().unwrap();

            if x == x2
            {
                if y < y2
                {
                    for y in y..=y2
                    { grid[y as usize][x as usize - min_x as usize] = Spot::Rock; }
                }
                else
                {
                    for y in y2..=y
                    { grid[y as usize][x as usize - min_x as usize] = Spot::Rock; }
                }
            }
            else if y == y2
            {
                if x < x2
                {
                    for x in x..=x2
                    { grid[y as usize][x as usize - min_x as usize] = Spot::Rock; }
                }
                else
                {
                    for x in x2..=x
                    { grid[y as usize][x as usize - min_x as usize] = Spot::Rock; }
                }
            }
        }
    }

    for x in 0..grid[0].len()
    { grid[max_y as usize][x] = Spot::Rock; }

    let mut num_sand_pieces = 0;
    'outer: loop
    {
        let mut x = 500 - min_x;
        let mut y = 0;
        loop
        {
            if grid[(y + 1) as usize][x as usize] == Spot::Empty
            { y += 1; }
            else if grid[(y + 1) as usize][(x - 1) as usize] == Spot::Empty
            { y += 1; x -= 1; }
            else if grid[(y + 1) as usize][(x + 1) as usize] == Spot::Empty
            { y += 1; x += 1; }
            else
            {
                grid[y as usize][x as usize] = Spot::Sand;
                num_sand_pieces += 1;

                if x == 500 - min_x && y == 0
                { break 'outer; }
                else
                { break; }
            }
        }
    }

    // print the grid
    for y in 0..grid.len()
    {
        for x in 0..grid[0].len()
        {
            match grid[y][x]
            {
                Spot::Empty => print!("."),
                Spot::Rock => print!("#"),
                Spot::Sand => print!("o"),
            }
        }
        println!();
    }

    num_sand_pieces
}