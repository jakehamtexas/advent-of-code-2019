use std::fs;

fn main() {
    let inputs_raw = read_from_file("../../input.txt");
    let masses = parse_as_vector(inputs_raw);
    let total_fuel = get_total_fuel_from_masses(masses);

    println!("{}", total_fuel);
}

fn read_from_file(relative_input_path: &str) -> String {
    fs::read_to_string(relative_input_path).expect("input.txt doesn't exist!")
}

fn parse_as_vector(inputs_raw: String) -> Vec<i32> {
    inputs_raw
        .split_ascii_whitespace()
        .map(|m| m.parse().unwrap())
        .collect()
}

fn get_total_fuel_from_masses(masses: Vec<i32>) -> i32 {
    masses
        .iter()
        .fold(0, |total_fuel, mass| total_fuel + (mass / 3) - 2)
}
