use std::collections::HashMap;
use cached::proc_macro::cached;
use cached::SizedCache;
use regex::Regex;
use crate::tasks::helper::get_lines;

#[derive(Clone)]
struct Operation
{
	result: Option<usize>,
	operation: char,
	monkey1: String,
	monkey2: String,
}

#[derive(Clone)]
enum Task
{
	Operation(Operation),
	Number(usize),
}

#[cached(
	type = "SizedCache<String, usize>",
	create = "{ SizedCache::with_size(1000) }",
	convert = r#"{ monkey.to_string() }"#
)]
fn evaluate_monkey(monkey: &str, monkeys: &HashMap<String, Task>) -> usize
{
	match monkeys.get(monkey).unwrap()
	{
		Task::Operation(operation) =>
		{
			let value1 = evaluate_monkey(&operation.monkey1, monkeys);
			let value2 = evaluate_monkey(&operation.monkey2, monkeys);
			match operation.operation
			{
				'+' => value1 + value2,
				'-' => value1 - value2,
				'*' => value1 * value2,
				'/' => value1 / value2,
				_ => panic!("Invalid operation: {}", operation.operation),
			}
		},
		Task::Number(number) => *number,
	}
}

fn percolate_down(monkey: &str, monkeys: &mut HashMap<String, Task>)
{
	let monkey = monkeys.get_mut(monkey).unwrap().clone();
	match monkey
	{
		Task::Operation(operation) =>
		{
			let value1 = evaluate_monkey_top_down(&operation.monkey1, monkeys);
			let value2 = evaluate_monkey_top_down(&operation.monkey2, monkeys);
			match operation.operation
			{
				'=' => match (value1, value2)
				{
					//only one monkey has a value, but they must be equal,
					//so set the result of the other monkey to the value of the first
					(Some(value1), None) => {
						let monkey2 = monkeys.get_mut(&operation.monkey2).unwrap();
						match monkey2
						{
							Task::Operation(operation) => operation.result = Some(value1),
							Task::Number(number) => *number = value1,
						}

						percolate_down(&operation.monkey2, monkeys);
					}
					(None, Some(value2)) => {
						let monkey1 = monkeys.get_mut(&operation.monkey1).unwrap();
						match monkey1
						{
							Task::Operation(operation) => operation.result = Some(value2),
							Task::Number(number) => *number = value2,
						}

						percolate_down(&operation.monkey1, monkeys);
					}
					_ => (),
				},
				'+' => match (value1, value2)
				{
					(Some(value1), None) => {
						let monkey2 = monkeys.get_mut(&operation.monkey2).unwrap();
						match monkey2
						{
							Task::Operation(operation2) => operation2.result = Some(operation.result.unwrap() - value1),
							Task::Number(number) => *number = operation.result.unwrap() - value1,
						}

						percolate_down(&operation.monkey2, monkeys);
					},
					(None, Some(value2)) => {
						let monkey1 = monkeys.get_mut(&operation.monkey1).unwrap();
						match monkey1
						{
							Task::Operation(operation1) => operation1.result = Some(operation.result.unwrap() - value2),
							Task::Number(number) => *number = operation.result.unwrap() - value2,
						}

						percolate_down(&operation.monkey1, monkeys);
					},
					_ => (),
				},
				'-' => match (value1, value2)
				{
					(Some(value1), None) => {
						let monkey2 = monkeys.get_mut(&operation.monkey2).unwrap();
						match monkey2
						{
							Task::Operation(operation2) => operation2.result = Some(value1 - operation.result.unwrap()),
							Task::Number(number) => *number = value1 - operation.result.unwrap(),
						}

						percolate_down(&operation.monkey2, monkeys);
					},
					(None, Some(value2)) => {
						let monkey1 = monkeys.get_mut(&operation.monkey1).unwrap();
						match monkey1
						{
							Task::Operation(operation1) => operation1.result = Some(value2 + operation.result.unwrap()),
							Task::Number(number) => *number = value2 + operation.result.unwrap(),
						}

						percolate_down(&operation.monkey1, monkeys);
					},
					_ => (),
				},
				'*' => match (value1, value2)
				{
					(Some(value1), None) => {
						let monkey2 = monkeys.get_mut(&operation.monkey2).unwrap();
						match monkey2
						{
							Task::Operation(operation2) => operation2.result = Some(operation.result.unwrap() / value1),
							Task::Number(number) => *number = operation.result.unwrap() / value1,
						}

						percolate_down(&operation.monkey2, monkeys);
					},
					(None, Some(value2)) => {
						let monkey1 = monkeys.get_mut(&operation.monkey1).unwrap();
						match monkey1
						{
							Task::Operation(operation1) => operation1.result = Some(operation.result.unwrap() / value2),
							Task::Number(number) => *number = operation.result.unwrap() / value2,
						}

						percolate_down(&operation.monkey1, monkeys);
					},
					_ => (),
				},
				'/' => match (value1, value2)
				{
					(Some(value1), None) => {
						let monkey2 = monkeys.get_mut(&operation.monkey2).unwrap();
						match monkey2
						{
							Task::Operation(operation2) => operation2.result = Some(value1 / operation.result.unwrap()),
							Task::Number(number) => *number = value1 / operation.result.unwrap(),
						}

						percolate_down(&operation.monkey2, monkeys);
					},
					(None, Some(value2)) => {
						let monkey1 = monkeys.get_mut(&operation.monkey1).unwrap();
						match monkey1
						{
							Task::Operation(operation1) => operation1.result = Some(value2 * operation.result.unwrap()),
							Task::Number(number) => *number = value2 * operation.result.unwrap(),
						}

						percolate_down(&operation.monkey1, monkeys);
					},
					_ => (),
				},
				_ => panic!("Unknown operation: {}", operation.operation),
			}
		},
		Task::Number(_) => (),
	}
}

fn evaluate_monkey_top_down(monkey: &str, monkeys: &mut HashMap<String, Task>) -> Option<usize>
{
	match monkey
	{
		"humn" => None,
		name => match monkeys.get(name).unwrap().to_owned()
		{
			Task::Operation(operation) =>
			{
				let value1 = evaluate_monkey_top_down(&operation.monkey1, monkeys);
				let value2 = evaluate_monkey_top_down(&operation.monkey2, monkeys);
				match (value1, value2)
				{
					(Some(value1), Some(value2)) => match operation.operation {
						'+' => Some(value1 + value2),
						'-' => Some(value1 - value2),
						'*' => Some(value1 * value2),
						'/' => Some(value1 / value2),
						_ => panic!("Invalid operation: {}", operation.operation),
					},
					_ => None,
				}
			},
			Task::Number(number) => Some(number),
		},
	}
}

fn get_monkeys(filename: &str) -> HashMap<String, Task>
{
	let lines = get_lines(filename);
	let mut monkeys = HashMap::new();
	let operation_pattern = Regex::new(r"([a-z]{4}): ([a-z]{4}) ([+*/-]) ([a-z]{4})").unwrap();
	let number_pattern = Regex::new(r"([a-z]{4}): (\d+)").unwrap();
	for line in lines
	{
		if let Some(captures) = operation_pattern.captures(&line)
		{
			let operation = Operation
			{
				result: None,
				operation: captures[3].chars().next().unwrap(),
				monkey1: captures[2].to_string(),
				monkey2: captures[4].to_string(),
			};

			monkeys.insert(captures[1].to_string(), Task::Operation(operation));
		}
		else if let Some(captures) = number_pattern.captures(&line)
		{ monkeys.insert(captures[1].to_string(), Task::Number(captures[2].parse().unwrap())); }
	}

	monkeys
}

///The input is a file containing a list of monkeys and their given task.
///Each task is either a mathematical operation, or a number.
///The operations operate on the values provided by two other monkeys.
///This function returns the value that the monkey named "root" will produce.
pub fn get_root_value(filename: &str) -> usize
{
	let monkeys = get_monkeys(filename);

	evaluate_monkey("root", &monkeys)
}

///The operation for the root monkey was actually supposed to be "=".
///This function returns what the monkey named "humn" (human) should yell for both
///values passed to root to be equal.
pub fn get_input_value(filename: &str) -> usize
{
	let mut monkeys = get_monkeys(filename);

	let left_monkey = match monkeys.get("root").unwrap()
	{
		Task::Operation(operation) => operation.monkey1.clone(),
		_ => panic!("Root monkey is not an operation"),
	};
	let right_monkey = match monkeys.get("root").unwrap()
	{
		Task::Operation(operation) => operation.monkey2.clone(),
		_ => panic!("Root monkey is not an operation"),
	};

	monkeys.insert("humn".to_string(), Task::Number(50000));
	monkeys.insert("root".to_string(), Task::Operation(Operation
	{
		result: None,
		operation: '=',
		monkey1: left_monkey.to_string(),
		monkey2: right_monkey.to_string(),
	}));

	percolate_down("root", &mut monkeys);

	match monkeys["humn"]
	{
		Task::Number(number) => number,
		_ => panic!("Humn monkey is not a number"),
	}
}