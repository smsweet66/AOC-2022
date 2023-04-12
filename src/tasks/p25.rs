use crate::tasks::helper::get_lines;

fn base_10_to_5(input: i64) -> String {
	let mut result = String::new();
	let mut remainder = input;
	while remainder != 0 {
		match remainder % 5 {
			0 => result.push('0'),
			1 => result.push('1'),
			2 => result.push('2'),
			3 => {
				result.push('=');
				remainder += 5;
			}
			4 => {
				result.push('-');
				remainder += 5;
			}
			_ => panic!("Invalid remainder in base 5 conversion"),
		}
		remainder /= 5;
	}

	result.chars().rev().collect()
}

fn base_5_to_10(input: &str) -> i64 {
	let mut result = 0;
	let mut multiplier = 1;
	for c in input.chars().rev() {
		match c {
			'=' => result -= 2 * multiplier,
			'-' => result -= multiplier,
			'0' => (),
			'1' => result += multiplier,
			'2' => result += 2 * multiplier,
			_ => panic!("Invalid character in base 5 number"),
		}
		multiplier *= 5;
	}

	result
}

///The input is a list of numbers given in base 5.  However, rather than
///being expressed using the digits 0-4, they are expressed using the
///digits =-2.  = represents -2, - represents -1, and the digits 0-2
///are unchanged.  This function calculates the sum of each number
///in the list, and returns the sum in base 5 as per the rules above.
pub fn sum_base_5(filename: &str) -> String {
	let lines = get_lines(filename);
	let sum = lines.iter().map(|line| base_5_to_10(line)).sum();

	return base_10_to_5(sum);
}