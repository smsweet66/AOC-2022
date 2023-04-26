use std::collections::VecDeque;
use crate::tasks::helper::get_lines;

#[derive(Default)]
struct Monkey<'a> {
	items: VecDeque<u64>,       // the number assigned to each item the monkey holds
	num_inspections: u64,       // the number of times the monkey has inspected an item
	operation: Vec<&'a str>,    // the operation performed on each item per iteration
	test: u64,                  // if the item value after the operation is divisible by this number, the test passes
	test_pass: usize,           // what to do if the test passes
	test_fail: usize,           // what to do if the test fails
}

impl Monkey<'_> {
	fn perform_operation(&mut self, test_multiplied: u64) -> Vec<(usize, u64)> {
		let mut result: Vec<(usize, u64)> = Vec::new();
		while self.items.len() > 0 {
			self.num_inspections += 1;
			let mut item = self.items.pop_front().unwrap();
			let a = match self.operation[0] {
				"old" => item,
				val => val.parse::<u64>().expect("Error parsing a"),
			};

			let b = match self.operation[2] {
				"old" => item,
				val => val.parse::<u64>().expect("Error parsing b"),
			};

			match self.operation[1] {
				"+" => item = (a + b) % test_multiplied,
				"*" => item = (a * b) % test_multiplied,
				error => panic!("Unknown operation: {}", error),
			}

			if item % self.test == 0 { result.push((self.test_pass, item)); }
			else { result.push((self.test_fail, item)); }
		}

		return result;
	}
}

///The input is a file containing a list of monkeys.  After creating the monkeys, the program
///will iterate through the monkeys and perform the operation on each monkey's items.  After 20
///iterations, the program will output the product of the number of times the top two monkeys have
///operated on an item.
pub fn get_monkey_business(filename: &str) -> u64 {
	let lines = get_lines(filename);
	let mut monkeys: Vec<Monkey> = Vec::new();
	let mut test_multiplied = 1; // the product of all the test numbers
	for line in &lines {
		let stripped_line = line.trim();
		let line_split: Vec<&str> = stripped_line.split(" ").collect();
		match line_split[0] {
			"Monkey" => monkeys.push(Monkey::default()),
			"Starting" => {
				let mut items: VecDeque<u64> = VecDeque::new();
				for item in &line_split[2..] {
					let mut item_string = item.to_string();
					if item_string.ends_with(",") { item_string.pop(); }

					items.push_back(item_string.parse::<u64>().expect("Error parsing item"));
				}
				monkeys.last_mut().unwrap().items = items;
			}
			"Operation:" => monkeys.last_mut().unwrap().operation = line_split[3..].to_vec(),
			"Test:" => {
				let test = line_split[3].parse::<u64>().expect("Error parsing test");
				test_multiplied *= test;
				monkeys.last_mut().unwrap().test = test;
			}
			"If" => {
				match line_split[1] {
					"true:" => monkeys.last_mut().unwrap().test_pass = line_split[5].parse::<usize>().expect("Error parsing test_pass"),
					"false:" => monkeys.last_mut().unwrap().test_fail = line_split[5].parse::<usize>().expect("Error parsing test_fail"),
					error => panic!("Unknown test: {error}"),
				}
			}
			"" => continue,
			error => panic!("Unknown line: {error}"),
		}
	}

	for _ in 0..10000 {
		for i in 0..monkeys.len() {
			let result = monkeys[i].perform_operation(test_multiplied);
			for (monkey_index, item) in result { monkeys[monkey_index].items.push_back(item); }
		}
	}

	let mut greatest = 0;
	let mut second_greatest = 0;
	for monkey in &monkeys {
		if monkey.num_inspections > greatest {
			second_greatest = greatest;
			greatest = monkey.num_inspections;
		} else if monkey.num_inspections > second_greatest {
			second_greatest = monkey.num_inspections;
		}
	}

	return greatest * second_greatest;
}