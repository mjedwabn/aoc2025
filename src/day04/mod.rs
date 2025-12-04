use std::{io::BufRead, ops::Add};

use itertools::Itertools;

use crate::{CartesianGrid, Coords, ICoords, read_input};

pub fn accessible_paper_rolls(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let printing_department = CartesianGrid::from(lines);

  printing_department.get_accessible_paper_rolls().len()
}

pub fn how_many_paper_rolls_can_be_removed(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let mut printing_department = CartesianGrid::from(lines);

  let mut total_removed_paper_rolls = 0;

  loop {
    let paper_rolls = printing_department.get_accessible_paper_rolls();

    if paper_rolls.len() == 0 {
      break;
    }

    printing_department.remove_paper_rolls(&paper_rolls);
    total_removed_paper_rolls += paper_rolls.len();
  }

  total_removed_paper_rolls
}

trait PrintingDepartment {
  fn get_adjacent_paper_rolls(&self, coord: &Coords) -> Vec<Coords>;
  fn get_accessible_paper_rolls(&self) -> Vec<Coords>;
  fn remove_paper_rolls(&mut self, paper_rolls: &Vec<Coords>);
}

impl PrintingDepartment for CartesianGrid<char> {
  fn get_accessible_paper_rolls(&self) -> Vec<Coords> {
    self
      .coords()
      .iter()
      .filter(|c| *self.get(c) == '@')
      .filter(|c| self.get_adjacent_paper_rolls(c).len() < 4)
      .map(|c| *c)
      .collect_vec()
  }

  fn remove_paper_rolls(&mut self, paper_rolls: &Vec<Coords>) {
    for pr in paper_rolls {
      self.set(pr, 'x');
    }
  }

  fn get_adjacent_paper_rolls(&self, coords: &Coords) -> Vec<Coords> {
    self
      .get_adjacent_coords_in_bounds(*coords)
      .iter()
      .filter(|c| *self.get(c) == '@')
      .map(|c| *c)
      .collect_vec()
  }
}

impl CartesianGrid<char> {
  fn get_adjacent_coords_in_bounds(&self, coords: Coords) -> Vec<Coords> {
    vec![
      coords.add(ICoords(-1, 1)),
      coords.add_y(1),
      coords.add(ICoords(1, 1)),
      coords.add_x(1),
      coords.add(ICoords(-1, -1)),
      coords.sub_y(1),
      coords.add(ICoords(1, -1)),
      coords.sub_x(1),
    ]
    .iter()
    .filter(|c| self.in_grid(c))
    .map(|c| c.to_coords().unwrap())
    .collect_vec()
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    day04::{accessible_paper_rolls, how_many_paper_rolls_can_be_removed},
    read,
  };

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      accessible_paper_rolls(&mut read("./src/day04/sample.input")),
      13
    );
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      accessible_paper_rolls(&mut read("./src/day04/my.input")),
      1435
    );
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(
      how_many_paper_rolls_can_be_removed(&mut read("./src/day04/sample.input")),
      43
    );
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      how_many_paper_rolls_can_be_removed(&mut read("./src/day04/my.input")),
      8623
    );
  }
}
