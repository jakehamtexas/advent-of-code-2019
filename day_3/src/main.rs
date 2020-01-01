use collections::HashMap;
use std::collections;

mod models;
use models::Direction;
use models::Ray;

extern crate input;
use input::*;

fn main() {
    let wires = get_wires();
    let coordinates_of_nearest_cross = get_coordinates_of_nearest_cross(&wires);
    let manhattan_distance = get_manhattan_distance_from_coordinates(coordinates_of_nearest_cross);
    println!("Manhattan Distance: {}", manhattan_distance);

    let traversed_points_by_wire = get_traversed_points_by_wire(&wires);
    let crosses = get_crosses_from_traversed_points(&traversed_points_by_wire);
    let mut crosses_by_traversed_steps =
        get_crosses_by_traversed_steps(crosses, traversed_points_by_wire.to_vec());

    crosses_by_traversed_steps.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let closest_cross = crosses_by_traversed_steps.first().unwrap();
    println!("Number steps to closest cross: {}", closest_cross.1);
}

fn get_wires() -> Vec<Vec<Ray>> {
    let inputs_raw = read_from_file("./input.txt");
    let wires_raw = parse_as_vector_from_whitespace::<String>(inputs_raw);
    models::parse_as_wires(wires_raw)
}

fn get_coordinates_of_nearest_cross(wires: &[Vec<Ray>]) -> [i32; 2] {
    let crosses = get_crosses_from_wires(wires);
    get_nearest_cross(crosses)
}

fn get_nearest_cross(mut crosses: Vec<[i32; 2]>) -> [i32; 2] {
    crosses.sort_by(|a, b| {
        get_manhattan_distance_from_coordinates(*a)
            .partial_cmp(&get_manhattan_distance_from_coordinates(*b))
            .unwrap()
    });
    *crosses.first().unwrap()
}

fn get_manhattan_distance_from_coordinates(coordinates: [i32; 2]) -> i32 {
    coordinates.iter().map(|coordinate| coordinate.abs()).sum()
}

fn get_crosses_from_wires(wires: &[Vec<Ray>]) -> Vec<[i32; 2]> {
    let traversed_points_by_wire = get_traversed_points_by_wire(&wires);
    get_crosses_from_traversed_points(&traversed_points_by_wire)
}

fn get_traversed_points_by_wire(wires: &[Vec<Ray>]) -> Vec<Vec<[i32; 2]>> {
    wires
        .iter()
        .map(|wire| get_traversed_points_for_wire(wire))
        .collect()
}

fn get_traversed_points_for_wire(wire: &[Ray]) -> Vec<[i32; 2]> {
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

fn get_points_from_ray_and_previous_point(ray: &Ray, previous_point: [i32; 2]) -> Vec<[i32; 2]> {
    let x = previous_point[0];
    let y = previous_point[1];

    (0..ray.distance)
        .map(|i| {
            let offset = i + 1;
            match ray.direction {
                Direction::Up => [x, y + offset],
                Direction::Down => [x, y - offset],
                Direction::Left => [x - offset, y],
                Direction::Right => [x + offset, y],
            }
        })
        .collect()
}

fn get_crosses_from_traversed_points(traversed_points_by_wire: &[Vec<[i32; 2]>]) -> Vec<[i32; 2]> {
    traversed_points_by_wire
        .iter()
        .fold(
            HashMap::new(),
            |occurrences_by_point: HashMap<[i32; 2], i32>, traversed_points| {
                aggregate_hashmaps(
                    occurrences_by_point,
                    get_occurrences_by_point(traversed_points),
                )
            },
        )
        .into_iter()
        .filter(|(_, number_of_occurences)| *number_of_occurences > 1)
        .map(|(point, _)| point)
        .collect()
}

fn aggregate_hashmaps(
    first: HashMap<[i32; 2], i32>,
    second: HashMap<[i32; 2], i32>,
) -> HashMap<[i32; 2], i32> {
    let mut new = first.clone();
    for (k, _) in second {
        *new.entry(k).or_default() += 1;
    }
    new
}

fn get_occurrences_by_point(traversed_points: &[[i32; 2]]) -> HashMap<[i32; 2], i32> {
    traversed_points.iter().fold(
        HashMap::new(),
        |mut occurrences_by_point: HashMap<[i32; 2], i32>, point| {
            occurrences_by_point.insert(*point, 1);
            occurrences_by_point
        },
    )
}

fn get_crosses_by_traversed_steps(
    crosses: Vec<[i32; 2]>,
    traversed_points_by_wire: Vec<Vec<[i32; 2]>>,
) -> Vec<([i32; 2], usize)> {
    crosses
        .into_iter()
        .map(|cross| {
            let sum_of_steps_from_all_wires = traversed_points_by_wire
                .iter()
                .map(|traversed_points| {
                    traversed_points
                        .iter()
                        .position(|point| point == &cross)
                        .unwrap()
                        + 1
                })
                .sum();
            (cross, sum_of_steps_from_all_wires)
        })
        .collect()
}
