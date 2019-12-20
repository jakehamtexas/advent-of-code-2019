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
    let coordinates_of_nearest_cross = get_coordinates_of_nearest_cross(wires);
    let manhattan_distance = get_manhattan_distance_from_coordinates(coordinates_of_nearest_cross);
    println!("Manhattan Distance: {}", manhattan_distance);
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

fn get_coordinates_of_nearest_cross(wires: Vec<Vec<Ray>>) -> [i32; 2] {
    let crosses = get_crosses_from_wires(wires);
    let nearest_cross = get_nearest_cross(crosses);
    return nearest_cross;
}

fn get_crosses_from_wires(wires: Vec<Vec<Ray>>) -> Vec<[i32; 2]> {
    let traversed_points_by_wire = wires
        .into_iter()
        .map(|wire| get_traversed_points_for_wire(wire))
        .collect();
    let crosses = get_crosses_from_traversed_points(traversed_points_by_wire);
    return crosses;
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
    traversed_points_by_wire: Vec<Vec<[i32; 2]>>,
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
fn get_nearest_cross(mut crosses: Vec<[i32; 2]>) -> [i32; 2] {
    crosses.sort_by(|a, b| {
        get_manhattan_distance_from_coordinates(*a)
            .partial_cmp(&get_manhattan_distance_from_coordinates(*b))
            .unwrap()
    });
    println!("Point: {:?}", crosses.first().unwrap());

    *crosses.first().unwrap()
}

fn get_manhattan_distance_from_coordinates(coordinates: [i32; 2]) -> i32 {
    coordinates
        .iter()
        .fold(0, |manhattan_distance, coordinate| {
            manhattan_distance + coordinate.abs()
        })
}
