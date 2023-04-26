use std::fs;

///The input is a file containing a single data stream.
///In order to find the packet within the data stream, we need to find the start
///of the packet, which is denoted by n different characters in a row.
///This function returns the index of the first character of the message.
pub fn find_packet_start(filename: &str, n: usize) -> usize {
	let data = fs::read_to_string(filename).expect("Unable to read file");
	let mut message_start = n;
	for i in 0..data.len() - n {
		let sub_message = &data[i..i + n];
		let mut found = true;
		for i in 1..n {
			if sub_message[i..].contains(sub_message.chars().nth(i - 1).unwrap()) {
				message_start += 1;
				found = false;
				break;
			}
		}

		if found { break; }
	}

	return message_start;
}