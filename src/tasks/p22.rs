use regex::Regex;
use crate::tasks::helper::get_lines;

#[derive(Debug, Clone, Copy)]
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
	let max_width = lines[..lines.len()-2].iter().map(|x| x.len()).max().unwrap();

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
	let movements = get_moves(&moves, &pattern);

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

	match facing
	{
		Direction::Right => 1000 * (row + 1) + 4 * (col + 1),
		Direction::Down => 1000 * (row + 1) + 4 * (col + 1) + 1,
		Direction::Left => 1000 * (row + 1) + 4 * (col + 1) + 2,
		Direction::Up => 1000 * (row + 1) + 4 * (col + 1) + 3,
	}
}

///hardcoded for the given file input
fn wrap_cube(row: usize, col: usize, direction: Direction) -> (usize, usize, Direction)
{
	let cube_row = row / 50;
	let cube_col = col / 50;
	let (new_cube_row, new_cube_col, new_direction) = match (cube_row, cube_col, direction)
	{
		(0, 1, Direction::Up) => (3, 0, Direction::Right),
		(0, 1, Direction::Left) => (2, 0, Direction::Right),
		(0, 2, Direction::Up) => (3, 0, Direction::Up),
		(0, 2, Direction::Right) => (2, 1, Direction::Left),
		(0, 2, Direction::Down) => (1, 1, Direction::Left),
		(1, 1, Direction::Right) => (0, 2, Direction::Up),
		(1, 1, Direction::Left) => (2, 0, Direction::Down),
		(2, 0, Direction::Up) => (1, 1, Direction::Right),
		(2, 0, Direction::Left) => (0, 1, Direction::Right),
		(2, 1, Direction::Right) => (0, 2, Direction::Left),
		(2, 1, Direction::Down) => (3, 0, Direction::Left),
		(3, 0, Direction::Right) => (2, 1, Direction::Up),
		(3, 0, Direction::Down) => (0, 2, Direction::Down),
		(3, 0, Direction::Left) => (0, 1, Direction::Down),
		_ => panic!("Invalid cube wrap!: {} {} {:?}", row, col, direction),
	};

	let (row_idx, col_idx) = (row % 50, col % 50);

	let i = match direction
	{
		Direction::Left => 49 - row_idx,
		Direction::Right => row_idx,
		Direction::Up => col_idx,
		Direction::Down => 49 - col_idx,
	};


	// find new idxes within the cube

	let new_row = match new_direction
	{
		Direction::Left => 49 - i,
		Direction::Right => i,
		Direction::Up => 49,
		Direction::Down => 0,
	};

	let new_col = match new_direction
	{
		Direction::Left => 49,
		Direction::Right => 0,
		Direction::Up => i,
		Direction::Down => 49 - i,
	};

	(new_cube_row * 50 + new_row, new_cube_col * 50 + new_col, new_direction)
}

fn draw_map(map: &Vec<Vec<Spot>>, row: usize, col: usize, facing: Direction)
{
	for i in 0..map.len()
	{
		for j in 0..map[0].len()
		{
			if i == row && j == col
			{
				match facing
				{
					Direction::Right => print!(">"),
					Direction::Down => print!("v"),
					Direction::Left => print!("<"),
					Direction::Up => print!("^"),
				}
			}
			else
			{
				match map[i][j]
				{
					Spot::Empty => print!("."),
					Spot::Wall => print!("#"),
					Spot::Void => print!(" "),
				}
			}
		}
		println!();
	}
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
	let movements = get_moves(&moves, &pattern);

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
							match map[row].get(col + 1).unwrap_or(&Spot::Void)
							{
								Spot::Empty => col = col + 1,
								Spot::Wall => {},
								Spot::Void => {
									let (new_row, new_col, new_facing) = wrap_cube(row, col, facing);
									if map[new_row][new_col] == Spot::Empty
									{
										row = new_row;
										col = new_col;
										facing = new_facing;
									}
								},
							}
						}
						Direction::Down => {
							match map.get(row + 1).unwrap_or(&vec![Spot::Void]).get(col).unwrap_or(&Spot::Void)
							{
								Spot::Empty => row = row + 1,
								Spot::Wall => {},
								Spot::Void => {
									let (new_row, new_col, new_facing) = wrap_cube(row, col, facing);
									if map[new_row][new_col] == Spot::Empty
									{
										row = new_row;
										col = new_col;
										facing = new_facing;
									}
								},
							}
						},
						Direction::Left => {
							if col == 0
							{
								let (new_row, new_col, new_facing) = wrap_cube(row, col, facing);
								if map[new_row][new_col] == Spot::Empty
								{
									row = new_row;
									col = new_col;
									facing = new_facing;
								}

								continue;
							}

							match map[row].get(col - 1).unwrap_or(&Spot::Void)
							{
								Spot::Empty => col = col - 1,
								Spot::Wall => {},
								Spot::Void => {
									let (new_row, new_col, new_facing) = wrap_cube(row, col, facing);
									if map[new_row][new_col] == Spot::Empty
									{
										row = new_row;
										col = new_col;
										facing = new_facing;
									}
								},
							}
						},
						Direction::Up => {
							if row == 0
							{
								let (new_row, new_col, new_facing) = wrap_cube(row, col, facing);
								if map[new_row][new_col] == Spot::Empty
								{
									row = new_row;
									col = new_col;
									facing = new_facing;
								}

								continue;
							}

							match map[row - 1][col]
							{
								Spot::Empty => row = row - 1,
								Spot::Wall => {},
								Spot::Void => {
									let (new_row, new_col, new_facing) = wrap_cube(row, col, facing);
									if map[new_row][new_col] == Spot::Empty
									{
										row = new_row;
										col = new_col;
										facing = new_facing;
									}
								},
							}
						},
					}
				}
			}
		}
	}

	match facing
	{
		Direction::Right => 1000 * (row + 1) + 4 * (col + 1),
		Direction::Down => 1000 * (row + 1) + 4 * (col + 1) + 1,
		Direction::Left => 1000 * (row + 1) + 4 * (col + 1) + 2,
		Direction::Up => 1000 * (row + 1) + 4 * (col + 1) + 3,
	}
}