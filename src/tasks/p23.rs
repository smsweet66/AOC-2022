use std::collections::{HashMap, VecDeque};
use crate::tasks::helper::get_lines;

enum Direction
{
	North,
	South,
	West,
	East,
}

fn get_elves(filename: &str) -> HashMap<(i32, i32), Option<(i32, i32)>>
{
	let lines = get_lines(filename);

	//hashmap of elf positions and next positions
	let mut elves: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
	for (i, line) in lines.iter().enumerate()
	{
		for (j, c) in line.chars().enumerate()
		{
			if c == '#'
			{ elves.insert((j as i32, i as i32), None); }
		}
	}

	elves
}

///The input is a file containing a grid of either . or # characters.
///The . represents an empty space, and the # represents an elf.
///The empty space continues past the edge of the grid in all directions to infinity.
///Each round of movement has two parts:  the first half where each elf checks
///in each of the eight adjacent and diagonal spaces to them, If there are no
///elves in any of the adjacent spaces, the elf does nothing for that round.
///Otherwise it will attempt to move one space in the first valid direction.
///First it will check the N, NE, and NW directions, moving north if all three
///are empty.  Next it will check the S, SE, and SW directions, moving south
///if all three are empty.  Next it will check the W, NW, and SW directions,
///moving west if all three are empty.  Finally it will check the E, NE, and
///SE directions, moving east if all three are empty.
///In the second half of the round, the elves will attempt to move to the next space.
///If multiple elves are moving to the same space, all elves in conflict will
///remain in their current space.
///After the round is complete, the order that the directions are checked is
///changed:  The first direction checked is moved to the end of the list.
///After ten rounds, this function returns the number of empty spaces in the
///smallest bounding box that contains all elves.
pub fn get_empty_space_count(filename: &str) -> usize
{
	let mut elves = get_elves(filename);
	let mut direction_queue = VecDeque::from(vec![Direction::North, Direction::South, Direction::West, Direction::East]);

	for _ in 0..10
	{
		//check for surrounding elves
		for (current_pos, _) in elves.clone().iter_mut()
		{
			let mut has_surrounding = false;
			'elf: for i in -1..=1
			{
				for j in -1..=1
				{
					if i == 0 && j == 0 { continue; }
					if elves.contains_key(&(current_pos.0 + i, current_pos.1 + j))
					{
						has_surrounding = true;
						break 'elf;
					}
				}
			}

			if !has_surrounding { continue; }

			//find next position
			for direction in direction_queue.iter()
			{
				match direction
				{
					Direction::North => {
						if !elves.contains_key(&(current_pos.0, current_pos.1 - 1)) &&
							!elves.contains_key(&(current_pos.0 - 1, current_pos.1 - 1)) &&
							!elves.contains_key(&(current_pos.0 + 1, current_pos.1 - 1))
						{
							elves.insert(*current_pos, Some((current_pos.0, current_pos.1 - 1)));
							break;
						}
					},
					Direction::South => {
						if !elves.contains_key(&(current_pos.0, current_pos.1 + 1)) &&
							!elves.contains_key(&(current_pos.0 - 1, current_pos.1 + 1)) &&
							!elves.contains_key(&(current_pos.0 + 1, current_pos.1 + 1))
						{
							elves.insert(*current_pos, Some((current_pos.0, current_pos.1 + 1)));
							break;
						}
					},
					Direction::West => {
						if !elves.contains_key(&(current_pos.0 - 1, current_pos.1)) &&
							!elves.contains_key(&(current_pos.0 - 1, current_pos.1 - 1)) &&
							!elves.contains_key(&(current_pos.0 - 1, current_pos.1 + 1))
						{
							elves.insert(*current_pos, Some((current_pos.0 - 1, current_pos.1)));
							break;
						}
					},
					Direction::East => {
						if !elves.contains_key(&(current_pos.0 + 1, current_pos.1)) &&
							!elves.contains_key(&(current_pos.0 + 1, current_pos.1 - 1)) &&
							!elves.contains_key(&(current_pos.0 + 1, current_pos.1 + 1))
						{
							elves.insert(*current_pos, Some((current_pos.0 + 1, current_pos.1)));
							break;
						}
					},
				}
			}

			match elves.get(current_pos).unwrap()
			{
				None => { continue; },
				_ => {},
			}

			//check for conflicts
			let mut conflict = false;
			for i in -1..=1
			{
				for j in -1..=1
				{
					if i + j == 0 || i + j > 1 || i + j < -1 { continue; }
					let next_pos = elves[current_pos].unwrap();
					let position = (next_pos.0 + i, next_pos.1 + j);
					if position == *current_pos { continue; }

					if elves.contains_key(&position)
					{
						if elves[&position] != None && elves[&position].unwrap() == elves[current_pos].unwrap()
						{
							elves.insert(position, None);
							conflict = true;
						}
					}
				}
			}

			if conflict { elves.insert(*current_pos, None); }
		}

		//move elves
		for (current_pos, next_pos) in elves.clone().iter_mut()
		{
			if next_pos.is_none() { continue; }
			elves.remove(current_pos);
			elves.insert(next_pos.unwrap(), None);
		}

		//rotate direction queue
		let direction = direction_queue.pop_front().unwrap();
		direction_queue.push_back(direction);
	}

	//print grid
	let mut min_x = i32::MAX;
	let mut max_x = i32::MIN;
	let mut min_y = i32::MAX;
	let mut max_y = i32::MIN;
	for (x, y) in elves.keys()
	{
		if *x < min_x { min_x = *x; }
		if *x > max_x { max_x = *x; }
		if *y < min_y { min_y = *y; }
		if *y > max_y { max_y = *y; }
	}

	return ((max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32) as usize;
}

///Returns the first round in which no elf moves.
pub fn get_first_empty_round(filename: &str) -> usize
{
	let mut elves = get_elves(filename);
	let mut direction_queue = VecDeque::from(vec![Direction::North, Direction::South, Direction::West, Direction::East]);

	let mut round = 1;

	loop
	{
		//check for surrounding elves
		for (current_pos, _) in elves.clone().iter_mut()
		{
			let mut has_surrounding = false;
			'elf: for i in -1..=1
			{
				for j in -1..=1
				{
					if i == 0 && j == 0 { continue; }
					if elves.contains_key(&(current_pos.0 + i, current_pos.1 + j))
					{
						has_surrounding = true;
						break 'elf;
					}
				}
			}

			if !has_surrounding { continue; }

			//find next position
			for direction in direction_queue.iter()
			{
				match direction
				{
					Direction::North => {
						if !elves.contains_key(&(current_pos.0, current_pos.1 - 1)) &&
							!elves.contains_key(&(current_pos.0 - 1, current_pos.1 - 1)) &&
							!elves.contains_key(&(current_pos.0 + 1, current_pos.1 - 1))
						{
							elves.insert(*current_pos, Some((current_pos.0, current_pos.1 - 1)));
							break;
						}
					},
					Direction::South => {
						if !elves.contains_key(&(current_pos.0, current_pos.1 + 1)) &&
							!elves.contains_key(&(current_pos.0 - 1, current_pos.1 + 1)) &&
							!elves.contains_key(&(current_pos.0 + 1, current_pos.1 + 1))
						{
							elves.insert(*current_pos, Some((current_pos.0, current_pos.1 + 1)));
							break;
						}
					},
					Direction::West => {
						if !elves.contains_key(&(current_pos.0 - 1, current_pos.1)) &&
							!elves.contains_key(&(current_pos.0 - 1, current_pos.1 - 1)) &&
							!elves.contains_key(&(current_pos.0 - 1, current_pos.1 + 1))
						{
							elves.insert(*current_pos, Some((current_pos.0 - 1, current_pos.1)));
							break;
						}
					},
					Direction::East => {
						if !elves.contains_key(&(current_pos.0 + 1, current_pos.1)) &&
							!elves.contains_key(&(current_pos.0 + 1, current_pos.1 - 1)) &&
							!elves.contains_key(&(current_pos.0 + 1, current_pos.1 + 1))
						{
							elves.insert(*current_pos, Some((current_pos.0 + 1, current_pos.1)));
							break;
						}
					},
				}
			}

			match elves.get(current_pos).unwrap()
			{
				None => { continue; },
				_ => {},
			}

			//check for conflicts (only need to check spots adjacent to the next position)
			let mut conflict = false;
			for i in -1..=1
			{
				for j in -1..=1
				{
					if i + j == 0 || i + j > 1 || i + j < -1 { continue; }
					let next_pos = elves[current_pos].unwrap();
					let position = (next_pos.0 + i, next_pos.1 + j);
					if position == *current_pos { continue; }

					if elves.contains_key(&position)
					{
						if elves[&position] != None && elves[&position].unwrap() == elves[current_pos].unwrap()
						{
							elves.insert(position, None);
							conflict = true;
						}
					}
				}
			}

			if conflict { elves.insert(*current_pos, None); }
		}

		let mut moved = false;
		//move elves
		for (current_pos, next_pos) in elves.clone().iter_mut()
		{
			if next_pos.is_none() { continue; }
			elves.remove(current_pos);
			elves.insert(next_pos.unwrap(), None);
			moved = true;
		}

		if !moved { break; }

		round += 1;

		//rotate direction queue
		let direction = direction_queue.pop_front().unwrap();
		direction_queue.push_back(direction);
	}

	return round;
}