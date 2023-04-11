use std::cmp::Reverse;
use std::collections::HashSet;
use std::hash::Hash;
use priority_queue::PriorityQueue;
use crate::tasks::helper::get_lines;

#[derive(Clone, PartialEq, Eq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Clone, PartialEq, Eq)]
struct State {
	walls: Vec<(u32, u32)>,
	blizzards: Vec<(u32, u32, Direction)>,
	size: (u32, u32),
	location: (u32, u32),
	steps: u32,
}

impl Hash for State {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.location.hash(state);
		self.steps.hash(state);
	}
}

fn update_blizzards(blizzards: &mut Vec<(u32, u32, Direction)>, size: (u32, u32)) {
	for blizzard in blizzards.iter_mut() {
		match blizzard.2 {
			Direction::Up => {
				if blizzard.1 > 1 {
					blizzard.1 -= 1;
				} else {
					blizzard.1 = size.1 - 2;
				}
			}
			Direction::Down => {
				if blizzard.1 < size.1 - 2 {
					blizzard.1 += 1;
				} else {
					blizzard.1 = 1;
				}
			}
			Direction::Left => {
				if blizzard.0 > 1 {
					blizzard.0 -= 1;
				} else {
					blizzard.0 = size.0 - 2;
				}
			}
			Direction::Right => {
				if blizzard.0 < size.0 - 2 {
					blizzard.0 += 1;
				} else {
					blizzard.0 = 1;
				}
			}
		}
	}
}

fn manhattan_distance(a: (u32, u32), b: (u32, u32)) -> u32 {
	return (a.0 as i32 - b.0 as i32).abs() as u32 + (a.1 as i32 - b.1 as i32).abs() as u32;
}

fn get_possible_moves(location: (u32, u32), walls: &Vec<(u32, u32)>, blizzards: &Vec<(u32, u32)>, size: (u32, u32)) -> Vec<(u32, u32)>
{
	let mut possible_moves: Vec<(u32, u32)> = Vec::new();
	if location.0 > 0 && !walls.contains(&(location.0 - 1, location.1)) && !blizzards.contains(&(location.0 - 1, location.1)) {
		possible_moves.push((location.0 - 1, location.1));
	}
	if location.0 < size.0 - 1 && !walls.contains(&(location.0 + 1, location.1)) && !blizzards.contains(&(location.0 + 1, location.1)) {
		possible_moves.push((location.0 + 1, location.1));
	}
	if location.1 > 0 && !walls.contains(&(location.0, location.1 - 1)) && !blizzards.contains(&(location.0, location.1 - 1)) {
		possible_moves.push((location.0, location.1 - 1));
	}
	if location.1 < size.1 - 1 && !walls.contains(&(location.0, location.1 + 1)) && !blizzards.contains(&(location.0, location.1 + 1)) {
		possible_moves.push((location.0, location.1 + 1));
	}
	if !blizzards.contains(&location) {
		possible_moves.push(location);
	}

	return possible_moves;
}

fn shortest_path(initial_state: State, end: (u32, u32)) -> Option<u32> {
	let mut queue: PriorityQueue<State, Reverse<u32>> = PriorityQueue::new();
	queue.push(initial_state.clone(), Reverse(manhattan_distance(initial_state.location, end)));

	let mut visited = HashSet::new();
	visited.insert(initial_state);

	let mut current_best = u32::MAX;

	while !queue.is_empty() {
		let (current_state, distance) = queue.pop().unwrap();
		if current_state.steps + distance.0 >= current_best {
			continue;
		}

		if current_state.location == end {
			current_best = current_best.min(current_state.steps);
			println!("Current best: {}", current_best);
			continue;
		}

		let mut next_blizzards = current_state.blizzards.clone();
		update_blizzards(&mut next_blizzards, current_state.size);
		let blizzard_locations: Vec<(u32, u32)> = next_blizzards.iter().map(|blizzard| (blizzard.0, blizzard.1)).collect();

		for move_to in get_possible_moves(current_state.location, &current_state.walls, &blizzard_locations, current_state.size) {
			let state = State {
				walls: current_state.walls.clone(),
				blizzards: next_blizzards.clone(),
				size: current_state.size,
				location: move_to,
				steps: current_state.steps + 1,
			};

			if visited.insert(state.clone()) {
				queue.push(state, Reverse(manhattan_distance(move_to, end)));
			}
		}
	}

	return if current_best == u32::MAX {
		None
	} else {
		Some(current_best)
	}
}

fn print_map(walls: &Vec<(u32, u32)>, blizzards: &Vec<(u32, u32, Direction)>, size: (u32, u32), location: (u32, u32)) {
	for y in 0..size.1 {
		for x in 0..size.0 {
			if (x, y) == location {
				print!("E");
			} else if walls.contains(&(x, y)) {
				print!("#");
			} else if blizzards.contains(&(x, y, Direction::Up)) {
				print!("^");
			} else if blizzards.contains(&(x, y, Direction::Down)) {
				print!("v");
			} else if blizzards.contains(&(x, y, Direction::Left)) {
				print!("<");
			} else if blizzards.contains(&(x, y, Direction::Right)) {
				print!(">");
			} else {
				print!(".");
			}
		}
		println!("");
	}
}

fn get_map_info(filename: &str) -> ((u32, u32), Vec<(u32, u32)>, Vec<(u32, u32, Direction)>) {
	let lines = get_lines(filename);
	let size = (lines[0].len() as u32, lines.len() as u32);
	let mut walls: Vec<(u32, u32)> = Vec::new();
	let mut blizzards: Vec<(u32, u32, Direction)> = Vec::new();
	for (y, line) in lines.iter().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c == '#' {
				walls.push((x as u32, y as u32));
			} else if c == '^' {
				blizzards.push((x as u32, y as u32, Direction::Up));
			} else if c == 'v' {
				blizzards.push((x as u32, y as u32, Direction::Down));
			} else if c == '<' {
				blizzards.push((x as u32, y as u32, Direction::Left));
			} else if c == '>' {
				blizzards.push((x as u32, y as u32, Direction::Right));
			}
		}
	}

	return (size, walls, blizzards);
}

///This function takes in a map of a valley.  The map contains empty spaces ('.'), walls ('#'),
///and blizzards ('^', 'v', '<', '>').  The blizzards move in the direction indicated by the
///arrow.  The player starts at the top left corner and must reach the bottom right corner.
///The player can move in any direction, but cannot move through walls or blizzards.
///Blizzards and the player move at the same time.  When a blizzard hits a wall, it reappears
///on the opposite side facing the same direction.  This function returns the minimum number of
///moves required to reach the bottom right corner.  Valid moves are up, down, left, right, and wait.
pub fn minimum_moves(filename: &str) -> u32 {
	let (size, walls, blizzards) = get_map_info(filename);

	let initial_state = State {
		walls,
		blizzards,
		size,
		location: (1, 0),
		steps: 0,
	};

	let end = (size.0 - 2, size.1 - 1);

	return shortest_path(initial_state, end).unwrap();
}

pub fn minimum_moves_round_trip(filename: &str) -> u32 {
	let (size, walls, mut blizzards) = get_map_info(filename);

	let initial_state = State {
		walls: walls.clone(),
		blizzards: blizzards.clone(),
		size,
		location: (1, 0),
		steps: 0,
	};

	let end = (size.0 - 2, size.1 - 1);
	let path_len = shortest_path(initial_state, end).unwrap();
	for _ in 0..path_len {
		update_blizzards(&mut blizzards, size);
	}

	let initial_state = State {
		walls: walls.clone(),
		blizzards: blizzards.clone(),
		size,
		location: end,
		steps: path_len,
	};

	let end = (1, 0);
	let backtrack = shortest_path(initial_state, end).unwrap();
	for _ in path_len..backtrack {
		update_blizzards(&mut blizzards, size);
	}

	let initial_state = State {
		walls,
		blizzards,
		size,
		location: (1, 0),
		steps: backtrack,
	};

	let end = (size.0 - 2, size.1 - 1);
	return shortest_path(initial_state, end).unwrap();
}