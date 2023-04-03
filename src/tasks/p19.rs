use std::collections::{HashMap, VecDeque};
use regex::Regex;
use crate::tasks::helper::get_lines;

struct Blueprint
{
	id: usize,
	costs_per_robot: [[usize; 3]; 4] //ore, clay, obsidian cost for each of the four robot types
}

struct State
{
	robots: [usize; 4],
	inventory: [usize; 4],
	remaining_time: usize
}

impl Blueprint
{
	fn from_line(line: &str) -> Blueprint
	{
		let pattern = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

		let captures = pattern.captures(line).expect(&*("Invalid line format: ".to_owned() + line));
		let id = captures[1].parse::<usize>().unwrap();
		let ore_cost = captures[2].parse::<usize>().unwrap();
		let clay_cost = captures[3].parse::<usize>().unwrap();
		let obsidian_cost = captures[4].parse::<usize>().unwrap();
		let obsidian_clay_cost = captures[5].parse::<usize>().unwrap();
		let geode_cost = captures[6].parse::<usize>().unwrap();
		let geode_obsidian_cost = captures[7].parse::<usize>().unwrap();

		Blueprint
		{
			id,
			costs_per_robot: [[ore_cost, 0, 0], [clay_cost, 0, 0], [obsidian_cost, obsidian_clay_cost, 0], [geode_cost, 0, geode_obsidian_cost]]
		}
	}
}

fn max_geodes(blueprint: &Blueprint, time: usize) -> usize
{
	let mut max_robots = [usize::MAX; 4];
    for i in 0..3
    { max_robots[i] = blueprint.costs_per_robot.iter().map(|cost| cost[i]).max().unwrap(); }

	let mut max_geodes = 0;
	let mut queue = VecDeque::new();
	queue.push_back(State{robots: [1, 0, 0, 0], inventory: [0, 0, 0, 0], remaining_time: time});
	while let Some(State { robots, inventory, remaining_time }) = queue.pop_front()
	{
		for i in 0..4
		{
			// check if building the given robot is overkill
			if robots[i] >= max_robots[i]
			{ continue; }

			let costs = &blueprint.costs_per_robot[i];
			// get time required to get resources to build the given robot type
			let mut time_to_build = 1;
			for j in 0..3
			{
				if costs[j] > inventory[j] && robots[j] == 0
				{
					time_to_build = time + 1;
					break;
				}
				else if costs[j] > inventory[j]
				{ time_to_build = time_to_build.max(1 + (costs[j] - inventory[j] + robots[j] - 1) / robots[j]); }
			}

			// if we can't build the robot, skip it
			if 1 + time_to_build > remaining_time
			{ continue; }

			// get the new inventory and remaining time
			let mut new_inventory = [0; 4];
			for i in 0..3
			{ new_inventory[i] = inventory[i] + robots[i] * time_to_build - costs[i]; }

			new_inventory[3] = inventory[3] + robots[3] * time_to_build;

			let new_remaining_time = remaining_time - time_to_build;
			let mut new_robots = robots;
			new_robots[i] += 1;

			// add the new state to the queue
			queue.push_back(State{robots: new_robots, inventory: new_inventory, remaining_time: new_remaining_time});
		}

		max_geodes = max_geodes.max(inventory[3] + robots[3] * remaining_time);
	}

	return max_geodes;
}

///This function takes in a file containing a list of blueprints for
///4 different types of robots.  Each blueprint lists the cost of
///each robot.  It takes one minute to build a robot, and after a robot
///is built, it produces one unit of its resource type every minute.
///The different types of robots are:
///    - 1: ore robot (costs ore)
///    - 2: clay robot (costs ore)
///    - 3: obsidian robot (costs ore and clay)
///    - 4: geode robot (costs ore and obsidian)
///You start with a single ore robot.
///This function returns the sum of the number of geodes that can be produced
///in 24 minutes times the blueprint number.
pub fn get_sum_geodes(filename: &str) -> usize
{
	let lines = get_lines(filename);
	let mut blueprints = Vec::new();
	for line in &lines
	{ blueprints.push(Blueprint::from_line(line)); }

	let mut sum_quality = 0;
	for blueprint in &blueprints
	{
		let geodes_produced = max_geodes(&blueprint, 24);

		println!("Blueprint {} produces {} geodes", blueprint.id, geodes_produced);

		sum_quality += geodes_produced * blueprint.id;
	}

	sum_quality
}

pub fn get_product_geodes(filename: &str) -> usize
{
	let lines = get_lines(filename);
	let mut blueprints = Vec::new();
	for line in &lines[..3]
	{ blueprints.push(Blueprint::from_line(line)); }

	let mut geodes_product = 1;
	for blueprint in &blueprints
	{
		let geodes_produced = max_geodes(&blueprint, 32);

		println!("Blueprint {} produces {} geodes", blueprint.id, geodes_produced);

		geodes_product *= geodes_produced;
	}

	geodes_product
}