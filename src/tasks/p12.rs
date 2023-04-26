use std::cmp::Reverse;
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use crate::tasks::helper::get_lines;

fn get_manhattan_distance(pos1: (i32, i32), pos2: (i32, i32)) -> i32 {
	return (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs();
}

fn get_elevation(c: char) -> u32 {
	return if c == 'S' { 0 }
	else if c == 'E' { 25 }
	else { (c as u32) - ('a' as u32) }
}

fn get_valid_moves(map: &Vec<String>, pos: (i32, i32)) -> Vec<(i32, i32)> {
	let mut moves: Vec<(i32, i32)> = Vec::new();
	if pos.0 > 0 && get_elevation(map[pos.0 as usize - 1].chars().nth(pos.1 as usize).unwrap()) <= get_elevation(map[pos.0 as usize].chars().nth(pos.1 as usize).unwrap()) + 1
	{ moves.push((pos.0 - 1, pos.1)); }
	if pos.0 < map.len() as i32 - 1 && get_elevation(map[pos.0 as usize + 1].chars().nth(pos.1 as usize).unwrap()) <= get_elevation(map[pos.0 as usize].chars().nth(pos.1 as usize).unwrap()) + 1
	{ moves.push((pos.0 + 1, pos.1)); }
	if pos.1 > 0 && get_elevation(map[pos.0 as usize].chars().nth(pos.1 as usize - 1).unwrap()) <= get_elevation(map[pos.0 as usize].chars().nth(pos.1 as usize).unwrap()) + 1
	{ moves.push((pos.0, pos.1 - 1)); }
	if pos.1 < map[pos.0 as usize].len() as i32 - 1 && get_elevation(map[pos.0 as usize].chars().nth(pos.1 as usize + 1).unwrap()) <= get_elevation(map[pos.0 as usize].chars().nth(pos.1 as usize).unwrap()) + 1
	{ moves.push((pos.0, pos.1 + 1)); }

	return moves;
}

///Given a map, the starting position, and the final position, this function returns the length
///of the shortest path from the starting position to the final position using A*.
fn path_len(map: &Vec<String>, start_pos: (i32, i32), final_pos: (i32, i32)) -> Option<u32> {
	// stores all possible moves and their manhattan distances from the final position
	let mut possible_moves: PriorityQueue<(i32, i32), Reverse<i32>> = PriorityQueue::new();

	// stores each position and its travel distance from the starting position
	let mut distance_map: HashMap<(i32, i32), i32> = HashMap::new();

	// stores each position and its parent
	let mut parents: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

	possible_moves.push(start_pos, Reverse(get_manhattan_distance(start_pos, final_pos)));
	distance_map.insert(start_pos, 0);
	parents.insert(start_pos, start_pos);
	while possible_moves.len() > 0 {
		let current_pos = possible_moves.pop().unwrap().0;
		if current_pos == final_pos { break; }

		for next_pos in get_valid_moves(map, current_pos) {
			let next_distance = distance_map[&current_pos] + 1;
			if next_distance < *distance_map.get(&next_pos).unwrap_or(&i32::MAX) {
				distance_map.insert(next_pos, next_distance);
				parents.insert(next_pos, current_pos);
				possible_moves.push(next_pos, Reverse(next_distance + get_manhattan_distance(next_pos, final_pos)));
			}
		}
	}

	return if !distance_map.contains_key(&final_pos) { None }
	else { Some(distance_map[&final_pos] as u32) }
}

///The input is a file containing an elevation map of a region.
///The elevation is given as a character a-z, where a is the lowest
///elevation and z is the highest.  The Starting point is represented with
///an 'S' and the end point is represented with an 'E'.  'S' and 'E' have the
///elevations 'a' and 'z' respectively.  When moving from one square to an adjacent square,
///the elevation must be at most 1 unit higher than the current square.
///this function returns the length of the shortest path from 'S' to 'E'.
pub fn get_shortest_path_len(filename: &str) -> u32 {
	let map = get_lines(filename);
	let mut start_pos: (i32, i32) = (0, 0);
	let mut final_pos: (i32, i32) = (0, 0);
	'outer: for i in 0..map.len() {
		for j in 0..map[i].len() {
			if map[i].chars().nth(j).unwrap() == 'S' { start_pos = (i as i32, j as i32); }
			else if map[i].chars().nth(j).unwrap() == 'E' { final_pos = (i as i32, j as i32); }

			if start_pos != (0, 0) && final_pos != (0, 0) { break 'outer; }
		}
	}

	return path_len(&map, start_pos, final_pos).unwrap();
}

pub fn get_fewest_steps(filename: &str) -> u32 {
	let map = get_lines(filename);
	let mut start_positions: Vec<(i32, i32)> = Vec::new();
	let mut final_pos: (i32, i32) = (0, 0);
	for i in 0..map.len() {
		for j in 0..map[i].len() {
			let c = map[i].chars().nth(j).unwrap();
			if c == 'S' || c == 'a' { start_positions.push((i as i32, j as i32)); }
			else if c == 'E' { final_pos = (i as i32, j as i32); }
		}
	}

	let mut shortest_path_len = u32::MAX;
	for start_pos in start_positions {
		let path_len = path_len(&map, start_pos, final_pos);
		shortest_path_len = path_len.unwrap_or(u32::MAX).min(shortest_path_len);
	}

	return shortest_path_len;
}