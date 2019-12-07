use std::fs;

fn main() {
    let inputs_raw = read_from_file();
    let masses = parse_as_vector(inputs_raw);
    let total_fuel = get_total_fuel_from_masses(masses);

    println!("{}", total_fuel);
}

fn read_from_file() -> String {
    return fs::read_to_string("../../input.txt").expect("input.txt doesn't exist!");
}

fn parse_as_vector(inputs_raw: String) -> Vec<i32> {
    return inputs_raw
        .split_ascii_whitespace()
        .map(|m| m.parse().unwrap())
        .collect();
}

fn get_total_fuel_from_masses(masses: Vec<i32>) -> i32 {
    return masses
        .iter()
        .fold(0, |total_fuel, mass| total_fuel + (mass / 3) - 2);
}
