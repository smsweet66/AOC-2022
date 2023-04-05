use regex::Regex;
use crate::tasks::helper::get_lines;

enum Direction
{
	Right,
	Down,
	Left,
	Up,
}

enum Movement
{
	Forward(usize),
	Right,
	Left,
}

fn get_moves(moves: &str, pattern: &Regex) -> Vec<Movement>
{
	let mut movements = Vec::new();
	for cap in pattern.captures_iter(moves)
	{
		match &cap[1]
		{
			"R" => movements.push(Movement::Right),
			"L" => movements.push(Movement::Left),
			_ => movements.push(Movement::Forward(cap[1].parse().unwrap())),
		}
	}

	movements
}

#[derive(PartialEq)]
enum Spot
{
	Empty,
	Wall,
	Void,
}

fn build_map(filename: &str) -> (Vec<Vec<Spot>>, String)
{
	let lines = get_lines(filename);
	let mut max_width = lines[..lines.len()-2].iter().map(|x| x.len()).max().unwrap();

	let mut map = Vec::new();
	for line in lines[..lines.len()-2].iter()
	{
		let mut row = Vec::new();
		for c in line.chars()
		{
			match c
			{
				'.' => row.push(Spot::Empty),
				'#' => row.push(Spot::Wall),
				' ' => row.push(Spot::Void),
				_ => panic!("Invalid character in map"),
			}
		}
		while row.len() < max_width
		{ row.push(Spot::Void); }

		map.push(row);
	}

	(map, lines[lines.len()-1].clone())
}

///The input is a file containing a map of an area.
///"." represents an empty space, "#" represents a wall, and " " represents
///the void.  Below the map is a list of movements, which are either
///a number representing forward movement, or "R" or "L" representing
///right or left turns.  If a forward movement would cause you to hit a
///wall, the movement is stopped and the next move starts.  If a movement
///would cause you to hit the void, the movement wraps around to the other side,
///unless the movement would cause you to hit a wall, in which case the movement
///is stopped and the next move starts.
///after all movements are complete, this function returns the sum of
///1000 times the row, 4 times the column, and the facing (0 for right, 1 for down,
///2 for left, 3 for up) which represents the password.
pub fn get_password(filename: &str) -> usize
{
	let (map, moves) = build_map(filename);
	let pattern = Regex::new(r"(\d+|[RL])").unwrap();
	let mut movements = get_moves(&moves, &pattern);

	let mut row = 0;
	let mut col = 0;
	let mut facing = Direction::Right;
	//get initial column by finding first open space in first row
	for i in 0..map[0].len()
	{
		if map[0][i] == Spot::Empty
		{
			col = i;
			break;
		}
	}

	for movement in movements
	{
		match movement
		{
			Movement::Left => {
				match facing
				{
					Direction::Right => facing = Direction::Up,
					Direction::Down => facing = Direction::Right,
					Direction::Left => facing = Direction::Down,
					Direction::Up => facing = Direction::Left,
				}
			},
			Movement::Right => {
				match facing
				{
					Direction::Right => facing = Direction::Down,
					Direction::Down => facing = Direction::Left,
					Direction::Left => facing = Direction::Up,
					Direction::Up => facing = Direction::Right,
				}
			},
			Movement::Forward(distance) => {
				for _ in 0..distance
				{
					match facing
					{
						Direction::Right => {
							match map[row][(col+1)%map[0].len()]
							{
								Spot::Empty => col = (col+1)%map[0].len(),
								Spot::Wall => break,
								Spot::Void => {
									for i in 0..map[0].len()
									{
										match map[row][i]
										{
											Spot::Empty => {
												col = i;
												break;
											},
											Spot::Wall => break,
											Spot::Void => {},
										}
									}
								},
							}
						},
						Direction::Down => {
							match map[(row+1)%map.len()][col]
							{
								Spot::Empty => row = (row+1)%map.len(),
								Spot::Wall => break,
								Spot::Void => {
									for i in 0..map.len()
									{
										match map[i][col]
										{
											Spot::Empty => {
												row = i;
												break;
											},
											Spot::Wall => break,
											Spot::Void => {},
										}
									}
								},
							}
						},
						Direction::Left => {
							match map[row][(col+map[0].len()-1)%map[0].len()]
							{
								Spot::Empty => col = (col+map[0].len()-1)%map[0].len(),
								Spot::Wall => break,
								Spot::Void => {
									for i in (0..map[0].len()).rev()
									{
										match map[row][i]
										{
											Spot::Empty => {
												col = i;
												break;
											},
											Spot::Wall => break,
											Spot::Void => {},
										}
									}
								},
							}
						},
						Direction::Up => {
							match map[(row+map.len()-1)%map.len()][col]
							{
								Spot::Empty => row = (row+map.len()-1)%map.len(),
								Spot::Wall => break,
								Spot::Void => {
									for i in (0..map.len()).rev()
									{
										match map[i][col]
										{
											Spot::Empty => {
												row = i;
												break;
											},
											Spot::Wall => break,
											Spot::Void => {},
										}
									}
								},
							}
						},
					}
				}
			}
		}
	}

	let mut facing_num = 0;
	match facing
	{
		Direction::Right => facing_num = 0,
		Direction::Down => facing_num = 1,
		Direction::Left => facing_num = 2,
		Direction::Up => facing_num = 3,
	}

	1000 * (row + 1) + 4 * (col + 1) + facing_num
}

///Same as the above function, but the wrapping works differently.
///The map is actually split up into six sections, representing the
///faces of a cube.  When you hit the void, you wrap around to the
///corresponding face on the other side of the cube.  This can result
///in a change in facing.
/// ef
/// d
///bc
///a
///The above is a representation of a cube.  The faces are labeled
///with letters, and the void is represented by a space.  Moving right
///from a will result in moving to c, changing the facing from right
///to up.  Moving right from b will result in moving to c as usual.
///Moving right from c will result in moving to f, changing the facing
///from right to left.  Moving right from d will result in moving to
///f, changing the facing from right to up.
///The password is calculated the same way as the above function.
pub fn get_password_cube(filename: &str) -> usize
{
	let (map, moves) = build_map(filename);
	let pattern = Regex::new(r"(\d+|[RL])").unwrap();
	let mut movements = get_moves(&moves, &pattern);

	let mut row = 0;
	let mut col = 0;
	let mut facing = Direction::Right;
	//get initial column by finding first open space in first row
	for i in 0..map[0].len()
	{
		if map[0][i] == Spot::Empty
		{
			col = i;
			break;
		}
	}



	0
}