use regex::Regex;
use crate::tasks::helper::get_lines;

struct Sensor
{
    x: i128,
    y: i128,
    radius: i128,
}

/*
 * The input is a list of sensor locations and the location of the beacon closest to them.
 * This distance is measured using the Manhattan distance.
 * This function returns the number of places in a given row that a beacon can't be placed,
 * because it would be closer to a sensor than the beacon closest to it.
 */
pub fn get_num_invalid_locations(filename: &str, row: i128) -> usize
{
    let lines = get_lines(filename);
    let pattern = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: Vec<(i128, i128)> = Vec::new();
    let mut min_x = i128::MAX;
    let mut max_x = i128::MIN;
    for line in &lines
    {
        let captures = pattern.captures(line).expect(&*("Invalid input: ".to_string() + line));
        let sensor_location: (i128, i128) = (captures[1].parse().unwrap(), captures[2].parse().unwrap());
        let beacon_location: (i128, i128) = (captures[3].parse().unwrap(), captures[4].parse().unwrap());

        let radius = (beacon_location.0 - sensor_location.0).abs() + (beacon_location.1 - sensor_location.1).abs();
        sensors.push(Sensor { x: sensor_location.0, y: sensor_location.1, radius });
        beacons.push(beacon_location);

        if sensor_location.0 - radius < min_x
        { min_x = sensor_location.0 - radius; }
        if sensor_location.0 + radius > max_x
        { max_x = sensor_location.0 + radius; }
    }

    let mut invalid_locations = 0;
    for i in min_x..=max_x
    {
        if beacons.contains(&(i, row))
        { continue; }

        for sensor in &sensors
        {
            let distance = (sensor.x - i).abs() + (sensor.y - row).abs();
            if distance <= sensor.radius
            {
                invalid_locations += 1;
                break;
            }
        }
    }

    invalid_locations
}

pub fn get_beacon_location(filename: &str, row: i128) -> i128
{
    let lines = get_lines(filename);
    let pattern = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in &lines
    {
        let captures = pattern.captures(line).expect(&*("Invalid input: ".to_string() + line));
        let sensor_location: (i128, i128) = (captures[1].parse().unwrap(), captures[2].parse().unwrap());
        let beacon_location: (i128, i128) = (captures[3].parse().unwrap(), captures[4].parse().unwrap());

        let radius = (beacon_location.0 - sensor_location.0).abs() + (beacon_location.1 - sensor_location.1).abs();
        sensors.push(Sensor { x: sensor_location.0, y: sensor_location.1, radius });
    }

    let mut possible_locations: Vec<(i128, i128)> = Vec::new();
    for sensor in &sensors  // adds locations that are just outside of the sensor's radius
    {
        possible_locations.push((sensor.x - sensor.radius - 1, sensor.y));
        possible_locations.push((sensor.x + sensor.radius + 1, sensor.y));
        for x in sensor.x - sensor.radius..=sensor.x + sensor.radius
        {
            let x_distance = (x - sensor.x).abs();
            let y_distance = sensor.radius - x_distance + 1;
            possible_locations.push((x, sensor.y - y_distance));
            possible_locations.push((x, sensor.y + y_distance));
        }
    }

    possible_locations.retain(|location| location.0 >= 0 && location.0 <= row*2 && location.1 >= 0 && location.1 <= row*2);

    println!("Created set of possible locations");
    'location: for location in &possible_locations
    {
        for sensor in &sensors
        {
            let distance = (sensor.x - location.0).abs() + (sensor.y - location.1).abs();
            if distance <= sensor.radius
            { continue 'location; }
        }

        return location.0 * 4000000 + location.1;
    }

    -1
}