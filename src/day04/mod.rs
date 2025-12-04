use std::{io::BufRead, ops::Add};

use itertools::Itertools;

use crate::{CartesianGrid, Coords, ICoords, read_input};

pub fn accessible_paper_rolls(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let grid = CartesianGrid::from(lines);

  grid
    .coords()
    .iter()
    .filter(|c| *grid.get(c) == '@')
    .filter(|c| grid.get_adjacent_paper_rolls(c).len() < 4)
    .count()
}

trait PrintingDepartment {
  fn get_adjacent_paper_rolls(&self, coord: &Coords) -> Vec<Coords>;
}

impl PrintingDepartment for CartesianGrid<char> {
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
  use crate::{day04::accessible_paper_rolls, read};

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
      13
    );
  }
}
