use std::collections::HashMap;
use cached::proc_macro::cached;
use cached::SizedCache;
use crate::tasks::helper::get_lines;

#[derive(Debug, Clone)]
struct Valve
{
    flow_rate: u32,
    connections: Vec<String>,
}

fn get_moves(current_valve: &str, valves: &HashMap<String, Valve>, visited: &mut Vec<String>, remaining_time: u32) -> Vec<String>
{
    let mut moves = Vec::new();
    for (name, valve) in valves
    {
        if valve.flow_rate == 0
        { continue; }

        if visited.contains(&name)
        { continue; }

        let distance = distance_to(current_valve, name, valves);
        if distance + 1 <= remaining_time
        { moves.push(name.clone()); }
    }

    moves
}

/*
 * Recursively checks all moves from the current valve to find the maximum pressure that can be released.
 */
fn test_all_moves_recursive(current_valve: (&str, &str), valves: &HashMap<String, Valve>, visited: &mut Vec<String>, remaining_time: (u32, u32)) -> u32
{
    let mut max_pressure = 0;
    let my_moves = get_moves(current_valve.0, valves, visited, remaining_time.0);
    let other_moves = get_moves(current_valve.1, valves, visited, remaining_time.1);
    for my_move in &my_moves
    {
        visited.push(my_move.clone());
        for other_move in &other_moves
        {
            if visited.contains(other_move)
            { continue; }

            visited.push(other_move.clone());
            let new_remaining_time = (remaining_time.0 - distance_to(current_valve.0, &my_move, valves) - 1,
                                     remaining_time.1 - distance_to(current_valve.1, other_move, valves) - 1);
            let pressure = valves[my_move].flow_rate * new_remaining_time.0 + valves[other_move].flow_rate * new_remaining_time.1;
            let new_pressure = pressure + test_all_moves_recursive((&my_move, other_move), valves, visited, new_remaining_time);
            if new_pressure > max_pressure
            { max_pressure = new_pressure; }
            visited.pop();
        }
        visited.pop();
    }

    // if other runs out of moves, but I still have moves, then I can just keep going
    if other_moves.len() == 0 && my_moves.len() > 0
    {
        for my_move in &my_moves
        {
            visited.push(my_move.clone());
            let new_remaining_time = (remaining_time.0 - distance_to(current_valve.0, &my_move, valves) - 1,
                                     remaining_time.1);
            let pressure = valves[my_move].flow_rate * new_remaining_time.0;
            let new_pressure = pressure + test_all_moves_recursive((&my_move, current_valve.1), valves, visited, new_remaining_time);
            if new_pressure > max_pressure
            { max_pressure = new_pressure; }
            visited.pop();
        }
    }
    else if my_moves.len() == 0 && other_moves.len() > 0
    {
        for other_move in &other_moves
        {
            visited.push(other_move.clone());
            let new_remaining_time = (remaining_time.0,
                                     remaining_time.1 - distance_to(current_valve.1, other_move, valves) - 1);
            let pressure = valves[other_move].flow_rate * new_remaining_time.1;
            let new_pressure = pressure + test_all_moves_recursive((current_valve.0, other_move), valves, visited, new_remaining_time);
            if new_pressure > max_pressure
            { max_pressure = new_pressure; }
            visited.pop();
        }
    }

    max_pressure
}

fn test_all_moves(valves: &HashMap<String, Valve>, remaining_time: (u32, u32)) -> u32
{
    let mut visited = Vec::new();
    test_all_moves_recursive(("AA", "AA"), valves, &mut visited, remaining_time)
}


fn distance_to_recursive(a: &str, b: &str, valves: &HashMap<String, Valve>, visited: &mut Vec<String>) -> u32
{
    if valves[a].connections.contains(&b.to_string())
    { 1 }
    else
    {
        let mut min_distance = u32::MAX;
        for connection in &valves[a].connections
        {
            if visited.contains(connection)
            { continue; }

            visited.push(connection.clone());
            let distance = distance_to_recursive(connection, b, valves, visited);
            if distance < min_distance
            { min_distance = distance; }
            visited.pop();
        }

        if min_distance == u32::MAX
        { u32::MAX }
        else
        { min_distance + 1 }
    }
}

#[cached(
    type = "SizedCache<String, u32>",
    create = "{ SizedCache::with_size(1000) }",
    convert = r#"{a.to_string() + b}"#
)]
fn distance_to(a: &str, b: &str, valves: &HashMap<String, Valve>) -> u32
{
    let mut visited = Vec::new();
    distance_to_recursive(a, b, valves, &mut visited)
}

/*
 * The input is a file containing a list of valves, their flow rate if opened (per minute),
 * and valves that you could access from the current valve.
 * Opening a valve takes 1 minute, as does moving from one valve to another.
 * This function returns the maximum pressure that can be released from the system.
 */
pub fn get_max_pressure(filename: &str) -> u32
{
    let lines = get_lines(filename);
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in &lines
    {
        let new_line = line.replace(";", "").replace(",", "");
        let mut parts = new_line.split_whitespace().collect::<Vec<&str>>();
        let name = parts[1].to_string();
        let flow_rate = parts[4][5..].parse::<u32>().unwrap();
        let mut connections = Vec::new();
        for valve in &parts[9..]
        { connections.push(valve.to_string()); }

        valves.insert(name, Valve { flow_rate, connections });
    }

    test_all_moves(&valves, (26, 26))
}