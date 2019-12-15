pub struct Ray {
  pub distance: i32,
  pub direction: Direction,
}

pub fn parse_as_ray(value: &str) -> Ray {
  let first_char = value.chars().next().unwrap();
  let direction = Direction::from_char_literal(first_char);

  let distance = value[1..].parse().unwrap();
  return Ray {
    distance: distance,
    direction: direction,
  };
}

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
