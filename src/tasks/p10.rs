use crate::tasks::helper::get_lines;

///The input is a file containing a series of operations, one per line.
///Each operation is either an addition or a noop.
///noop takes one cycle to complete, addition takes 2 cycles to complete.
///noop does nothing to the register, addition adds the given value to the register.
///From the register and the cycle number, we can calculate the signal strength by
///multiplying the cycle number by the register value.
///This function returns the sum of the signal strengths at the 20th, 60th,
///100th, 140th, 180th, and 220th cycle.
pub fn get_signal_strengths(input_file: &str) -> i32 {
	let lines = get_lines(input_file);
	let mut sum = 0;
	let mut register = 1;
	let mut cycle = 1;
	for line in &lines {
		if (cycle-20) % 40 == 0 { sum += cycle * register; }

		let mut parts = line.split_whitespace();
		match parts.next() {
			Some("addx") => {
				cycle += 1;
				if (cycle-20) % 40 == 0 { sum += cycle * register; }
				register += parts.next().unwrap().parse::<i32>().unwrap();
				cycle += 1;
			},
			Some("noop") => cycle += 1,
			_ => panic!("Invalid operation")
		}
	}

	return sum;
}

///The input is the same as the above function, but now the register value represents the center
///of a three pixel wide sprite shown as ###.  Each cycle a pixel is drawn starting from the left
///if the sprite is currently one the location being drawn, it is filled in with a #, otherwise
///it is filled in with a .  This function returns the string output representing the resulting screen.
pub fn get_screen(input_file: &str) -> String {
	let lines = get_lines(input_file);
	let mut screen = String::new();
	let mut register: i32 = 1;
	let mut cycle: i32 = 0;
	for line in &lines {
		let mut parts = line.split_whitespace();
		match parts.next() {
			Some("addx") => {
				if (register - cycle).abs() <= 1 { screen.push('#'); }
				else { screen.push('.'); }

				cycle += 1;
				if cycle % 40 == 0 {
					cycle = 0;
					screen.push('\n');
				}

				if (register - cycle).abs() <= 1 { screen.push('#'); }
				else { screen.push('.'); }

				cycle += 1;
				if cycle % 40 == 0 {
					cycle = 0;
					screen.push('\n');
				}

				register += parts.next().unwrap().parse::<i32>().unwrap();
			},
			Some("noop") => {
				if (register - cycle).abs() <= 1 { screen.push('#'); }
				else { screen.push('.'); }

				cycle += 1;
				if cycle % 40 == 0 {
					cycle = 0;
					screen.push('\n');
				}
			},
			_ => panic!("Invalid operation")
		}
	}

	return screen;
}