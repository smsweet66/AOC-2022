use crate::tasks::helper::get_lines;

fn get_x_y_distance(segment_a: &(i32, i32), segment_b: &(i32, i32)) -> (i32, i32) {
	let x_distance = segment_a.0 - segment_b.0;
	let y_distance = segment_a.1 - segment_b.1;
	return (x_distance, y_distance);
}

///Updates the locations of each segment of the rop after the head has moved.
///If the segment is not adjacent to the previous segment, it is moved according to these rules:
///if the segment is still in the same row or column as the previous segment, it is moved in the
///direction of the previous segment.  If the segment is in a different row and column than the
///previous segment, it is moved diagonally one space in the direction of the previous segment.
fn update_rope_locations(rope: &mut Vec<(i32, i32)>) {
	for i in 1..rope.len() {
		let distance = get_x_y_distance(&rope[i-1], &rope[i]);
		if distance.0.abs() <= 1 && distance.1.abs() <= 1 { continue; }

		if distance.0 > 0 { rope[i].0 += 1; }
		else if distance.0 < 0 { rope[i].0 -= 1; }

		if distance.1 > 0 { rope[i].1 += 1; }
		else if distance.1 < 0 { rope[i].1 -= 1; }
	}
}

///The input of the file is a list of movements describing the movement of the head of a rope.
///The head of the rope can move in 4 directions: up, down, left and right.  The tail of the rope
///is always touching the head, either adjacent to it, diagonally adjacent to it, or overlapping
///on the same spot.  After each movement, the tail of the rope is updated to be touching the
///head in the same way as before.  The goal is to find the number of unique spots that the tail
///of the rope has been in.
pub fn get_unique_tail_spots(filename: &str, rope_length: usize) -> usize {
	let lines = get_lines(filename);
	let mut rope = vec![(0, 0); rope_length];
	let mut unique_spots = vec![(0, 0)];
	for line in lines {
		let direction = line.chars().nth(0).unwrap();
		let distance = line[2..].parse::<u32>().unwrap();
		match direction {
			'U' => {
				for _ in 0..distance {
					rope[0].1 += 1;
					update_rope_locations(&mut rope);

					if !unique_spots.contains(&rope[rope_length-1]) { unique_spots.push(rope[rope_length-1]); }
				}
			}
			'D' => {
				for _ in 0..distance {
					rope[0].1 -= 1;
					update_rope_locations(&mut rope);

					if !unique_spots.contains(&rope[rope_length-1]) { unique_spots.push(rope[rope_length-1]); }
				}
			}
			'L' => {
				for _ in 0..distance {
					rope[0].0 -= 1;
					update_rope_locations(&mut rope);

					if !unique_spots.contains(&rope[rope_length-1]) { unique_spots.push(rope[rope_length-1]); }
				}
			}
			'R' => {
				for _ in 0..distance {
					rope[0].0 += 1;
					update_rope_locations(&mut rope);

					if !unique_spots.contains(&rope[rope_length-1]) { unique_spots.push(rope[rope_length-1]); }
				}
			}
			_ => panic!("Invalid direction: {}", direction)
		}
	}

	return unique_spots.len();
}