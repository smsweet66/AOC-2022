use std::collections::VecDeque;
use regex::Regex;
use crate::tasks::helper::get_lines;

fn build_stacks(lines: &Vec<String>) -> (Vec<VecDeque<char>>, usize) {
	let mut stacks: Vec<VecDeque<char>> = Vec::new();
	let mut command_start_line = 0;
	for line in lines {
		command_start_line += 1;
		if line.chars().nth(1).unwrap().is_numeric() {
			for _ in 0..=(line.len()/4) { stacks.push(VecDeque::new()); }

			break;
		}
	}
	
	for line in &lines[..command_start_line] {
		for i in 0..=(line.len() / 4)
		{
			if line.chars().nth(i* 4 + 1).unwrap() == ' ' {
				continue;
			} else if line.chars().nth(i * 4 + 1).unwrap().is_alphabetic(){
				stacks[i].push_front(line.chars().nth(i * 4 + 1).unwrap());
			} else { break; }
		}
	}

	return (stacks, command_start_line + 1)
}

///Takes in a file that contains several stacks of crates and
///movements of crates between the different stacks.
///This function will return a string of the top crate in each stack
///after all movements have been made.
pub fn top_crate_stacks(filename: &str) -> String {
	let lines = get_lines(filename);
	let (mut stacks, command) = build_stacks(&lines);
	let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
	for line in &lines[command..] {
		let command = pattern.captures(line).unwrap();
		let box_count: usize = command[1].parse().unwrap();
		let from: usize = command[2].parse().unwrap();
		let to: usize = command[3].parse().unwrap();
		for _ in 0..box_count {
			let crate_top = stacks[from - 1].pop_back().unwrap();
			stacks[to - 1].push_back(crate_top);
		}
	}

	let mut result: String = String::new();
	for stack in stacks { result.push(stack[stack.len() - 1]); }

	return result;
}

///Takes in a file that contains several stacks of crates and
///movements of crates between the different stacks. When multiple crates are
///moved at a time, order is preserved, that is, the entire stack is moved at once.
///This function will return a string of the top crate in each stack
///after all movements have been made.
pub fn top_crate_stacks_ordered(filename: &str) -> String {
	let lines = get_lines(filename);
	let (mut stacks, command) = build_stacks(&lines);
	let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
	let mut temp_stack: Vec<char> = Vec::new();
	for line in &lines[command..] {
		let command = pattern.captures(line).unwrap();
		let box_count: usize = command[1].parse().unwrap();
		let from: usize = command[2].parse().unwrap();
		let to: usize = command[3].parse().unwrap();
		for _ in 0..box_count {
			let crate_top = stacks[from - 1].pop_back().unwrap();
			temp_stack.push(crate_top);
		}

		for _ in 0..temp_stack.len() {
			let crate_top = temp_stack.pop().unwrap();
			stacks[to - 1].push_back(crate_top);
		}
	}

	let mut result: String = String::new();
	for stack in stacks { result.push(stack[stack.len() - 1]); }

	return result;
}