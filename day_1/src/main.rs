extern crate input;

fn main() {
    let inputs_raw = input::read_from_file("./input.txt");
    let ship_masses = input::parse_as_vector_from_whitespace::<i32>(inputs_raw);
    let fuel_for_ships = get_fuel_for_ships(&ship_masses);
    let fuel_for_ships_and_fuel_mass = get_fuel_for_ships_and_fuel_mass(&ship_masses);
    println!("Total fuel for ship masses: {}", fuel_for_ships);
    println!(
        "Total fuel for ship masses (and fuel): {}",
        fuel_for_ships_and_fuel_mass
    );
}

fn get_fuel_for_ships(masses: &Vec<i32>) -> i32 {
    masses
        .iter()
        .fold(0, |total_fuel, mass| total_fuel + get_fuel_for_mass(*mass))
}

fn get_fuel_for_ships_and_fuel_mass(masses: &Vec<i32>) -> i32 {
    masses.iter().fold(0, |total_fuel, mass| {
        total_fuel + get_fuel_for_ship_and_fuel_mass(*mass)
    })
}

fn get_fuel_for_ship_and_fuel_mass(mass: i32) -> i32 {
    let fuel = get_fuel_for_mass(mass);
    if fuel > 0 {
        fuel + get_fuel_for_ship_and_fuel_mass(fuel)
    } else {
        0
    }
}
fn get_fuel_for_mass(mass: i32) -> i32 {
    (mass / 3) - 2
}
