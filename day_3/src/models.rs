#[derive(Debug)]
pub struct Ray {
  pub distance: i32,
  pub direction: Direction,
}

pub fn parse_as_wires(wires_raw: Vec<String>) -> Vec<Vec<Ray>> {
  wires_raw
    .iter()
    .map(|wire_raw| wire_raw.split(',').map(|m| parse_as_ray(m)).collect())
    .collect()
}

fn parse_as_ray(value: &str) -> Ray {
  let first_char = value.chars().next().unwrap();
  let direction = Direction::from_char_literal(first_char);
  let distance = value[1..].parse().unwrap();
  Ray {
    distance,
    direction,
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
  Left,
  Right,
  Up,
  Down,
}

impl Direction {
  pub fn from_char_literal(value: char) -> Direction {
    match value {
      'L' => Direction::Left,
      'R' => Direction::Right,
      'U' => Direction::Up,
      'D' => Direction::Down,
      _ => panic!("Unknown value: {}", value),
    }
  }
}
