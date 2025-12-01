use std::{io::BufRead};

use itertools::Itertools;

pub fn what_is_the_password_to_open_the_door(input: &mut dyn BufRead) -> usize {
  let rotations = parse_input(crate::read_input(input));
  let mut dial = Dial::new();

  rotations
    .iter()
    .map(|r| dial.rotate(*r))
    .filter(|(p, _)| *p == 0)
    .count()
}

pub fn what_is_the_password_to_open_the_door_using_password_method(
  input: &mut dyn BufRead,
) -> u32 {
  let rotations = parse_input(crate::read_input(input));

  how_many_times_did_dial_passed_zero(&rotations)
}

fn how_many_times_did_dial_passed_zero(rotations: &Vec<i32>) -> u32 {
  let mut dial = Dial::new();
  rotations
    .iter()
    .map(|r| dial.rotate(*r))
    .map(|(_, passed_zero)| passed_zero)
    .sum()
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

  fn rotate(&mut self, distance: i32) -> (u32, u32) {
    let before = self.position;
    let temporary_position = self.position as i32 + distance;
    self.position = ((temporary_position).rem_euclid((self.max_number + 1) as i32)) as u32;

    let passed_zero_times = (temporary_position / (self.max_number as i32 + 1)).abs() as u32 
      + if temporary_position <= 0 && before != 0 { 1 as u32 } else {0 as u32};

    (self.position, passed_zero_times)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    day01::{
      what_is_the_password_to_open_the_door,
      what_is_the_password_to_open_the_door_using_password_method,
    },
    read,
  };

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

  #[test]
  fn sample_part2_input() {
    assert_eq!(
      what_is_the_password_to_open_the_door_using_password_method(&mut read(
        "./src/day01/sample.input"
      )),
      6
    );
  }

  #[test]
  fn count_passed_zero() {
    assert_eq!(super::Dial::new().rotate(49), (99, 0));
    assert_eq!(super::Dial::new().rotate(50), (0, 1));
    assert_eq!(super::Dial::new().rotate(51), (1, 1));
    assert_eq!(super::Dial::new().rotate(-49), (1, 0));
    assert_eq!(super::Dial::new().rotate(-50), (0, 1));
    assert_eq!(super::Dial::new().rotate(-51), (99, 1));
    assert_eq!(super::Dial::new().rotate(49 + 100), (99, 1));
    assert_eq!(super::Dial::new().rotate(50 + 100), (0, 2));
    assert_eq!(super::Dial::new().rotate(51 + 100), (1, 2));
    assert_eq!(super::Dial::new().rotate(-49 - 100), (1, 1));
    assert_eq!(super::Dial::new().rotate(-50 - 100), (0, 2));
    assert_eq!(super::Dial::new().rotate(-51 - 100), (99, 2));
    assert_eq!(super::Dial::new().rotate(49 + 100 * 2), (99, 2));
    assert_eq!(super::Dial::new().rotate(50 + 100 * 2), (0, 3));
    assert_eq!(super::Dial::new().rotate(-49 - 100 * 2), (1, 2));
    assert_eq!(super::Dial::new().rotate(-50 - 100 * 2), (0, 3));
  }

  #[test]
  fn count_passed_zero_multiple_rotations() {
    assert_eq!(super::how_many_times_did_dial_passed_zero(&vec![-50, 1]), 1);
    assert_eq!(super::how_many_times_did_dial_passed_zero(&vec![-50, -1]), 1);
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      what_is_the_password_to_open_the_door_using_password_method(&mut read(
        "./src/day01/my.input"
      )),
      6228
    );
  }
}
