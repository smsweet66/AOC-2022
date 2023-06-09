use crate::tasks::helper::get_lines;

///Given the strategy guide for the rock paper scissors matches,
///returns the score you would get if it were correct
///A/X = rock, B/Y = paper, C/Z = scissors
pub fn strategy_points(strategy_file: &str) -> u32 {
	let lines = get_lines(strategy_file);

	return lines.iter()
		.map(|line| {
			let opponent_move = line.chars().nth(0).unwrap() as u32;
			let my_move = line.chars().nth(2).unwrap() as u32 - 23;  //shifts moves from X, Y, and Z, to A, B, and C

			let score = my_move - 64; //calculates score from move
			let result = (3 + my_move - opponent_move)%3;
			match result
			{
				0 => score + 3,    //tie
				1 => score + 6,     //win
				_ => score
			}
		})
		.sum();
}

///Given the strategy guide for the rock paper scissors matches,
///returns the score you would get if it were correct
///A = rock, B = paper, C = scissors
///X = lose, Y = draw, Z = win
pub fn strategy_points_updated(strategy_file: &str) -> u32 {
	let lines = get_lines(strategy_file);

	return lines.iter()
		.map(|line| {
			let opponent_move = line.chars().nth(0).unwrap() as u32 - 65;
			let result = line.chars().nth(2).unwrap() as u32 - 88;

			(opponent_move + result + 2)%3 + 1 + result * 3
		})
		.sum();
}