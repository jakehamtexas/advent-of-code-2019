use collections::HashMap;
use std::collections;
use std::fs;
mod models;
use models::Direction;
use models::Ray;

fn main() {
    let inputs_raw = read_from_file();
    let wires_raw = get_wires_raw(inputs_raw);
    let wires = parse_as_wires(wires_raw);
    let traversed_points_for_wires = wires
        .into_iter()
        .map(|wire| get_traversed_points_for_wire(wire))
        .collect();
    let crosses = get_crosses_from_traversed_points(&traversed_points_for_wires);
    let mut crosses_by_traversed_steps =
        get_crosses_by_traversed_steps(crosses, traversed_points_for_wires);
    println!(
        "Number steps to closest cross: {:#?}",
        crosses_by_traversed_steps
    );

    crosses_by_traversed_steps.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let closest_cross = crosses_by_traversed_steps.first().unwrap();
    println!("Number steps to closest cross: {}", closest_cross.1);
}

fn get_wires_raw(inputs_raw: String) -> Vec<String> {
    return inputs_raw
        .split_ascii_whitespace()
        .map(|m| m.to_string())
        .collect();
}
fn read_from_file() -> String {
    return fs::read_to_string("../../input.txt").expect("input.txt doesn't exist!");
}

fn parse_as_wires(wires_raw: Vec<String>) -> Vec<Vec<Ray>> {
    return wires_raw
        .iter()
        .map(|wire_raw| parse_as_wire(wire_raw))
        .collect();
}

fn parse_as_wire(wire_raw: &String) -> Vec<Ray> {
    return wire_raw
        .split(",")
        .map(|m| models::parse_as_ray(m))
        .collect();
}

fn get_traversed_points_for_wire(wire: Vec<Ray>) -> Vec<[i32; 2]> {
    let mut traversed_points: Vec<[i32; 2]> = Vec::new();

    let starting_point = [0, 0];
    let mut previous_point = starting_point;
    for ray in wire {
        let points = get_points_from_ray_and_previous_point(ray, previous_point);
        traversed_points.extend_from_slice(&points);
        previous_point = *points.last().unwrap();
    }
    traversed_points
}

fn get_points_from_ray_and_previous_point(ray: Ray, previous_point: [i32; 2]) -> Vec<[i32; 2]> {
    let mut x = previous_point[0];
    let mut y = previous_point[1];

    let distance = ray.distance;
    let direction = ray.direction;

    let mut points = vec![[x, y]];
    for _ in 0..distance {
        match direction {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        };
        points.push([x, y]);
    }
    return points;
}

fn get_crosses_from_traversed_points(
    traversed_points_by_wire: &Vec<Vec<[i32; 2]>>,
) -> Vec<[i32; 2]> {
    traversed_points_by_wire
        .iter()
        .fold(
            HashMap::new(),
            |occurrences_by_point: HashMap<[i32; 2], i32>, traversed_points| {
                accumulate_hashmaps(
                    occurrences_by_point,
                    get_occurrences_by_point(traversed_points),
                )
            },
        )
        .into_iter()
        .filter(|(point, number_of_occurences)| *number_of_occurences > 1 && *point != [0, 0])
        .map(|(point, _number_of_occurences)| point)
        .collect()
}
fn accumulate_hashmaps(
    first: HashMap<[i32; 2], i32>,
    second: HashMap<[i32; 2], i32>,
) -> HashMap<[i32; 2], i32> {
    let mut new = first.clone();
    for (k, _) in second {
        *new.entry(k).or_default() += 1;
    }
    new
}

fn get_occurrences_by_point(traversed_points: &Vec<[i32; 2]>) -> HashMap<[i32; 2], i32> {
    let mut occurrences_by_point: HashMap<[i32; 2], i32> = HashMap::new();
    for point in traversed_points {
        occurrences_by_point.insert(*point, 1);
    }
    occurrences_by_point
}
fn get_crosses_by_traversed_steps(
    crosses: Vec<[i32; 2]>,
    traversed_points_by_wire: Vec<Vec<[i32; 2]>>,
) -> Vec<([i32; 2], i32)> {
    let mut crosses_by_traversed_steps: Vec<([i32; 2], i32)> = Vec::new();
    for cross in crosses.iter() {
        let mut traversed_steps: i32 = 0;
        for points in traversed_points_by_wire.iter() {
            traversed_steps += points.iter().position(|point| point == cross).unwrap() as i32;
        }
        crosses_by_traversed_steps.push((*cross, traversed_steps));
    }
    crosses_by_traversed_steps
}
