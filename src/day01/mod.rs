use std::io::BufRead;

use itertools::Itertools;

pub fn what_is_the_password_to_open_the_door(input: &mut dyn BufRead) -> usize {
  let lines = crate::read_input(input);
  let rotations = parse_input(lines);
  let mut dial = Dial::new();

  rotations
    .iter()
    .map(|r| dial.rotate(*r))
    .filter(|p| *p == 0)
    .count()
}

fn parse_input(lines: Vec<String>) -> Vec<i32> {
  fn parse_rotation(line: &str) -> i32 {
    let chars = line.chars().collect_vec();
    let direction = chars.get(0).unwrap();
    let distance: i32 = line[1..].parse().unwrap();

    match direction {
      'L' => -distance,
      'R' => distance,
      _ => panic!("Invalid direction"),
    }
  }

  lines.iter().map(|line| parse_rotation(line)).collect()
}

struct Dial {
  position: u32,
  max_number: u32,
}

impl Dial {
  fn new() -> Self {
    Dial {
      position: 50,
      max_number: 99,
    }
  }

  fn rotate(&mut self, distance: i32) -> u32 {
    self.position = ((self.position as i32 + distance).rem_euclid((self.max_number + 1) as i32)) as u32;
    self.position
  }
}

#[cfg(test)]
mod tests {
  use crate::{day01::what_is_the_password_to_open_the_door, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      what_is_the_password_to_open_the_door(&mut read("./src/day01/sample.input")),
      3
    );
  }
  
  #[test]
  fn my_part1_input() {
    assert_eq!(
      what_is_the_password_to_open_the_door(&mut read("./src/day01/my.input")),
      1036
    );
  }
}
