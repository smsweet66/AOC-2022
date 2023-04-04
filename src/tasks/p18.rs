use std::collections::HashMap;
use regex::Regex;
use crate::tasks::helper::get_lines;

fn get_cubes(filename: &str) -> Vec<(usize, usize, usize)>
{
	let lines = get_lines(filename);
	let pattern = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
	let mut cubes: Vec<(usize, usize, usize)> = Vec::new();
	for line in lines
	{
		let captures = pattern.captures(&line).unwrap();
		cubes.push((captures[1].parse().unwrap(), captures[2].parse().unwrap(), captures[3].parse().unwrap()));
	}

	cubes
}

fn get_uncovered_area_from_cubes(cubes: &Vec<(usize, usize, usize)>) -> usize
{
	let mut uncovered_area = cubes.len() * 6;
	for i in 0..cubes.len()
	{
		for j in i+1..cubes.len()
		{
			if cubes[i].0 == cubes[j].0 && cubes[i].1 == cubes[j].1
			{
				if cubes[i].2 == cubes[j].2 + 1 || cubes[i].2 == cubes[j].2 - 1
				{ uncovered_area -= 2; }
			}
			else if cubes[i].0 == cubes[j].0 && cubes[i].2 == cubes[j].2
			{
				if cubes[i].1 == cubes[j].1 + 1 || cubes[i].1 == cubes[j].1 - 1
				{ uncovered_area -= 2; }
			}
			else if cubes[i].1 == cubes[j].1 && cubes[i].2 == cubes[j].2
			{
				if cubes[i].0 == cubes[j].0 + 1 || cubes[i].0 == cubes[j].0 - 1
				{ uncovered_area -= 2; }
			}
		}
	}

	uncovered_area
}

/// given a file containing the x,y,z coordinates of many 1x1x1 cubes, find the number of
/// faces that aren't touching any other cube
pub fn get_uncovered_area(filename: &str) -> usize
{
	let cubes = get_cubes(filename);

	get_uncovered_area_from_cubes(&cubes)
}

fn get_neighbors(cube: &(usize, usize, usize), cubes: &Vec<(usize, usize, usize)>) -> Vec<(usize, usize, usize)>
{
	let mut neighbors: Vec<(usize, usize, usize)> = Vec::new();
	if !cubes.contains(&(cube.0 + 1, cube.1, cube.2))
	{ neighbors.push((cube.0 + 1, cube.1, cube.2)); }
	if !cubes.contains(&(cube.0 - 1, cube.1, cube.2))
	{ neighbors.push((cube.0 - 1, cube.1, cube.2)); }
	if !cubes.contains(&(cube.0, cube.1 + 1, cube.2))
	{ neighbors.push((cube.0, cube.1 + 1, cube.2)); }
	if !cubes.contains(&(cube.0, cube.1 - 1, cube.2))
	{ neighbors.push((cube.0, cube.1 - 1, cube.2)); }
	if !cubes.contains(&(cube.0, cube.1, cube.2 + 1))
	{ neighbors.push((cube.0, cube.1, cube.2 + 1)); }
	if !cubes.contains(&(cube.0, cube.1, cube.2 - 1))
	{ neighbors.push((cube.0, cube.1, cube.2 - 1)); }

	neighbors
}

/// returns whether the given cube is bounded by other cubes
fn is_bounded(cube: &(usize, usize, usize), cubes: &Vec<(usize, usize, usize)>,
              min_cube: &(usize, usize, usize), max_cube: &(usize, usize, usize)) -> bool
{
	// if the cube has a path to the edge of the structure, it is not bounded
	let mut queue: Vec<(usize, usize, usize)> = Vec::new();
	queue.push(*cube);
	let mut visited: Vec<(usize, usize, usize)> = Vec::new();
	while !queue.is_empty()
	{
		let current = queue.pop().unwrap();
		if current.0 <= min_cube.0 || current.0 >= max_cube.0 ||
		   current.1 <= min_cube.1 || current.1 >= max_cube.1 ||
		   current.2 <= min_cube.2 || current.2 >= max_cube.2
		{ return false; }

		for neighbor in get_neighbors(&current, cubes)
		{
			if !visited.contains(&neighbor)
			{ queue.push(neighbor); }
		}

		visited.push(current);
	}

	true
}

/// given a file containing the x,y,z coordinates of many 1x1x1 cubes, find the external
/// surface area of the resulting structure formed by the cubes
pub fn get_external_surface_area(filename: &str) -> usize
{
	let mut cubes = get_cubes(filename);

	let min_cube = (cubes.iter().min_by_key(|c| c.0).unwrap().0,
					cubes.iter().min_by_key(|c| c.1).unwrap().1,
					cubes.iter().min_by_key(|c| c.2).unwrap().2);

	let max_cube = (cubes.iter().max_by_key(|c| c.0).unwrap().0,
					cubes.iter().max_by_key(|c| c.1).unwrap().1,
					cubes.iter().max_by_key(|c| c.2).unwrap().2);

	let mut cubes_len = 0;
	let mut new_cubes_len = cubes.len();
	while cubes_len < new_cubes_len
	{
		let mut neighbors: Vec<(usize, usize, usize)> = Vec::new();
		for i in cubes_len..new_cubes_len
		{
			let mut new_neighbors = get_neighbors(&cubes[i], &cubes);
			for neighbor in &new_neighbors
			{
				if !neighbors.contains(neighbor)
				{ neighbors.push(*neighbor); }
			}
		}

		for neighbor in &neighbors
		{
			if is_bounded(neighbor, &cubes, &min_cube, &max_cube)
			{ cubes.push(*neighbor); }
		}

		cubes_len = new_cubes_len;
		new_cubes_len = cubes.len();
	}

	get_uncovered_area_from_cubes(&cubes)
}