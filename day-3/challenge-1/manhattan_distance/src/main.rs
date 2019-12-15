use std::fs;
mod models;
use models::Ray;

fn main() {
    let wires_raw: Vec<String> = read_from_file()
        .split_ascii_whitespace()
        .map(|m| m.to_string())
        .collect();
    let wires = parse_as_wires(wires_raw);
    let coordinates_of_nearest_cross = get_coordinates_of_nearest_cross(wires);
    let manhattan_distance = get_manhattan_distance_from_coordinates(coordinates_of_nearest_cross);
    println!("{}", manhattan_distance);
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
    return [0, 0];
}

fn get_manhattan_distance_from_coordinates(coordinates: [i32; 2]) -> i32 {
    return 0;
}
