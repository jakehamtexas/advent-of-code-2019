use std::fs::read_to_string;

pub fn read_from_file(relative_input_path: &str) -> String {
    read_to_string(relative_input_path).expect("Input doesn't exist!")
}

use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_as_vector_from_whitespace<T: FromStr>(inputs_raw: String) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    inputs_raw
        .split_ascii_whitespace()
        .map(|m| m.parse().expect("Value is not T parseable."))
        .collect()
}

pub fn parse_as_vector_from_delimiter<T: FromStr>(inputs_raw: String, delimiter: char) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    inputs_raw
        .split(delimiter)
        .map(|m| m.parse().expect("Value is not T parseable."))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
