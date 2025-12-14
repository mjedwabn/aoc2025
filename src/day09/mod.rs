use std::{cmp, io::BufRead};

use itertools::Itertools;

use crate::{read_input};

pub fn area_of_largest_rectangle(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let coords: Vec<Coords> = parse_coords(lines);

  coords.iter()
    .cartesian_product(coords.iter())
    .filter(|pair| pair.0 != pair.1)
    .map(|pair| (1 + cmp::max(pair.0.x, pair.1.x) - cmp::min(pair.0.x, pair.1.x)) * (1 + cmp::max(pair.0.y, pair.1.y) - cmp::min(pair.0.y, pair.1.y)))
    .max()
    .unwrap()
}

fn parse_coords(lines: Vec<String>) -> Vec<Coords> {
  lines.iter()
    .map(|line| line.split_once(','))
    .map(|split| Coords {
      x: split.unwrap().0.parse().unwrap(),
      y: split.unwrap().1.parse().unwrap()
    })
    .collect_vec()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Coords {
  x: usize,
  y: usize
}

#[cfg(test)]
mod tests {
  use crate::{day09::area_of_largest_rectangle, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      area_of_largest_rectangle(&mut read("./src/day09/sample.input")),
      50
    );
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      area_of_largest_rectangle(&mut read("./src/day09/my.input")),
      4737096935
    );
  }
}
