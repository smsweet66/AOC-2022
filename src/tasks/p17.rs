use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum RockType
{
	R1,
	R2,
	R3,
	R4,
	R5,
}

impl RockType
{
	fn value(&self) -> Vec<(usize, usize)>
	{
		match self
		{
			RockType::R1 => [(0, 0), (1, 0), (2, 0), (3, 0)].to_vec(),
			RockType::R2 => [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)].to_vec(),
			RockType::R3 => [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].to_vec(),
			RockType::R4 => [(0, 0), (0, 1), (0, 2), (0, 3)].to_vec(),
			RockType::R5 => [(0, 0), (1, 0), (0, 1), (1, 1)].to_vec(),
		}
	}
}

struct Rock
{
	rock_values: Vec<(usize, usize)>,
	location: (usize, usize),
}

impl Rock
{
	fn new(rock_type: RockType, location: (usize, usize)) -> Rock
	{
		Rock
		{
			rock_values: rock_type.value(),
			location,
		}
	}
}

fn get_max_height(column_heights: &[usize; 7]) -> usize
{
	let mut max_height = 0;
	for height in column_heights
	{ max_height = max_height.max(*height); }
	max_height
}

fn get_min_height(column_heights: &[usize; 7]) -> usize
{
	let mut min_height = usize::MAX;
	for height in column_heights
	{ min_height = min_height.min(*height); }
	min_height
}

fn is_valid_move(rock: &Rock, rock_locations: &Vec<(usize, usize)>) -> bool
{
	for(rock_x, rock_y) in &rock.rock_values
	{
		let x = rock.location.0 + rock_x;
		let y = rock.location.1 + rock_y;
		if x < 1 || x > 7 || y < 1 || rock_locations.contains(&(x, y))
		{ return false; }
	}

	return true;
}

#[derive(Hash, Eq, PartialEq)]
struct State
{
	column_heights_relative: [usize; 7],
	wind_index: usize,
	rock_type: RockType,
}

impl State
{
	fn new(column_heights: [usize; 7], wind_index: usize, rock_type: RockType) -> State
	{
		let mut column_heights_relative = [0; 7];
		let min_height = get_min_height(&column_heights);
		for i in 0..7
		{ column_heights_relative[i] = column_heights[i] - min_height; }

		State
		{
			column_heights_relative,
			wind_index,
			rock_type,
		}
	}
}

/// The input is a file containing the direction the wind blows after each downward
/// movement of the rock. The rocks fall within a 7 wide chamber and are spawned
/// so that the left edge is two spaces from the left of the chamber, and its bottom edge
/// is three spaces above the highest rock in the room.  After a rock spawns, it alternates
/// between being pushed left or right by the wind, and falling one space down.  If the rock
/// is pushed into the wall, the floor, or another rock, that movement is ignored.  If this occurs
/// during a downward movement, the rock comes to a stop and the next rock spawns.
/// The left wall is the 0th column, and the right wall is the 8th column.
/// This function returns height of the highest rock in the chamber after 2022 rocks have fallen.
pub fn get_rock_height(filename: &str, num_rocks: usize) -> usize
{
	let wind = fs::read_to_string(filename).unwrap().trim().to_string();
	let mut column_heights: [usize; 7] = [0; 7];
	let mut wind_index = 0;
	let mut rock_type = RockType::R1;
	let mut rock_locations = Vec::new();
	let mut states: HashMap<State, (usize, usize)> = HashMap::new();
	let mut num_rocks_fallen = 0;
	let mut skipped_height = 0;
	while num_rocks_fallen < num_rocks
	{
		let max_height = get_max_height(&column_heights);
		let mut rock = Rock::new(rock_type, (3, max_height + 4));
		loop
		{
			let wind_direction = wind.chars().nth(wind_index).unwrap();
			match wind_direction
			{
				'<' => rock.location.0 -= 1,
				'>' => rock.location.0 += 1,
				_ => panic!("Invalid wind direction"),
			}

			// if the move was invalid, undo it
			if !is_valid_move(&rock, &rock_locations)
			{
				match wind_direction
				{
					'<' => rock.location.0 += 1,
					'>' => rock.location.0 -= 1,
					_ => panic!("Invalid wind direction"),
				}
			}

			wind_index = (wind_index + 1) % wind.len();

			rock.location.1 -= 1;
			if !is_valid_move(&rock, &rock_locations)
			{
				rock.location.1 += 1;
				for (rock_x, rock_y) in &rock.rock_values
				{
					rock_locations.push((rock.location.0 + rock_x, rock.location.1 + rock_y));
					let x = rock.location.0 + rock_x;
					let y = rock.location.1 + rock_y;
					column_heights[x-1] = column_heights[x-1].max(y);
				}

				break;
			}
		}

		num_rocks_fallen += 1;
		if skipped_height == 0
		{
			let state = State::new(column_heights, wind_index, rock_type);
			if states.contains_key(&state)
			{
				let (old_num_rocks, old_height) = states.get(&state).unwrap();
				let num_rocks_per_cycle = num_rocks_fallen - old_num_rocks;
				let height_per_cycle = get_max_height(&column_heights) - old_height;
				let num_cycles = (num_rocks - num_rocks_fallen) / num_rocks_per_cycle;
				num_rocks_fallen += num_rocks_per_cycle * num_cycles;
				skipped_height = height_per_cycle * num_cycles;
			} else { states.insert(state, (num_rocks_fallen, get_max_height(&column_heights))); }
		}

		rock_type = match rock_type
		{
			RockType::R1 => RockType::R2,
			RockType::R2 => RockType::R3,
			RockType::R3 => RockType::R4,
			RockType::R4 => RockType::R5,
			RockType::R5 => RockType::R1,
		};
	}

	return get_max_height(&column_heights) + skipped_height;
}