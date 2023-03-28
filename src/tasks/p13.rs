use crate::tasks::helper::get_lines;

enum Packet
{
    Number(i32),
    Array(Vec<Packet>),
}

impl Packet
{
    fn compare(&self, other: &Packet) -> std::cmp::Ordering
    {
        match self
        {
            Packet::Number(n) => match other
            {
                Packet::Number(o) => n.cmp(o),
                Packet::Array(_) => {
                    let mut array: Vec<Packet> = Vec::new();
                    array.push(Packet::Number(*n));

                    Packet::Array(array).compare(other)
                },
            },
            Packet::Array(a) => match other
            {
                Packet::Number(n) => {
                    let mut array: Vec<Packet> = Vec::new();
                    array.push(Packet::Number(*n));

                    self.compare(&Packet::Array(array))
                },
                Packet::Array(o) => {
                    let compare_length = a.len().min(o.len());
                    for i in 0..compare_length
                    {
                        let result = a[i].compare(&o[i]);
                        if result != std::cmp::Ordering::Equal
                        { return result; }
                    }

                    return if a.len() > o.len()
                    { std::cmp::Ordering::Greater }
                    else if a.len() < o.len()
                    { std::cmp::Ordering::Less }
                    else
                    { std::cmp::Ordering::Equal }
                }
            }
        }
    }

    fn from_string(string: &str) -> Packet
    {
        if string.starts_with('[')
        {
            if string.len() == 2
            { return Packet::Array(Vec::new()); }

            let mut array = Vec::new();
            let mut current_value = String::new();
            let mut current_array_depth = 0;
            for c in string[1..string.len() - 1].chars()
            {
                match c
                {
                    '[' => {
                        current_array_depth += 1;
                        current_value.push(c);
                    },
                    ']' => {
                        current_array_depth -= 1;
                        current_value.push(c);
                    },
                    ',' => {
                        if current_array_depth == 0
                        {
                            array.push(Packet::from_string(&current_value));
                            current_value.clear();
                        }
                        else
                        { current_value.push(c); }
                    },
                    _ => current_value.push(c),
                }
            }
            array.push(Packet::from_string(&current_value));
            return Packet::Array(array);
        }
        else
        {
            match string.parse()
            {
                Ok(n) => return Packet::Number(n),
                Err(_) => panic!("Invalid number: {}", string),
            }
        }
    }

    fn to_string(&self) -> String
    {
        match self
        {
            Packet::Number(n) => n.to_string(),
            Packet::Array(a) =>
            {
                let mut string = String::new();
                string.push('[');
                for element in a
                {
                    string.push_str(&element.to_string());
                    string.push(',');
                }
                if string.len() > 1
                { string.pop(); }

                string.push(']');
                string
            },
        }
    }
}

/*
 * The input is a file containing several pairs of arrays.
 * Each pair is separated by a blank line.
 * Each Array element is either a Number or an Array.
 * This function returns the sum of all indices of packets
 * that are in the correct order.
 */
pub fn get_sum_correct_indices(filename: &str) -> u32
{
    let lines = get_lines(filename);
    let mut packets: Vec<Packet> = Vec::new();
    for line in &lines
    {
        if line.len() == 0
        { continue; }

        packets.push(Packet::from_string(line));
    }

    let mut index = 1;
    let mut sum = 0;
    for i in (0..packets.len() - 1).step_by(2)
    {
        let result = packets[i].compare(&packets[i + 1]);
        if result == std::cmp::Ordering::Less || result == std::cmp::Ordering::Equal
        { sum += index; }

        index += 1;
    }

    sum
}

/*
 * The input is the same as described as before, but now all packets must be sorted.
 * Additionally, two extra packets: [[2]] and [[6]] are added to the list of packets.
 * This function returns the product of the indices of the two new packets after sorting.
 */
pub fn sort_packets(filename: &str) -> u32
{
    let lines = get_lines(filename);
    let mut packets: Vec<Packet> = Vec::new();
    for line in &lines
    {
        if line.len() == 0
        { continue; }

        packets.push(Packet::from_string(line));
    }

    packets.push(Packet::from_string("[[2]]"));
    packets.push(Packet::from_string("[[6]]"));

    packets.sort_by(|a, b| a.compare(b));

    let mut index = 1;
    let mut product = 1;
    for packet in &packets
    {
        if packet.to_string() == "[[2]]"
        { product *= index; }
        else if packet.to_string() == "[[6]]"
        { product *= index; }

        index += 1;
    }

    product
}