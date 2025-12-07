use std::io::BufRead;

use crate::{CartesianGrid, Coords, GridCoords, read_input};

pub fn how_many_beam_splits(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let mut diagram = CartesianGrid::from(lines);
  let mut splits = 0;

  (0..diagram.height()).for_each(|y| splits += diagram.move_beams(&y));

  diagram.print();

  splits
}

trait BeamDiagram {
  fn move_beams(&mut self, y: &usize) -> usize;
  fn move_beam(&mut self, c: &Coords) -> usize;
  fn split_beam(&mut self, src: &Coords) -> usize;
}

impl BeamDiagram for CartesianGrid<char> {
  fn move_beams(&mut self, y: &usize) -> usize {
    let mut splits = 0;

    for c in self.coords_at_y(*y) {
      let u = self.get(&c);

      splits += match u {
        'S' => {
          self.set(&c.add_y(1).to_coords().unwrap(), '|');
          0
        }
        '|' => c
          .add_y(1)
          .to_coords()
          .map(|downstream| self.move_beam(&downstream))
          .unwrap_or(0),
        _ => 0,
      }
    }

    splits
  }

  fn move_beam(&mut self, c: &Coords) -> usize {
    if !c.in_grid(self) {
      return 0;
    }

    let u = self.get(c);

    match u {
      '^' => self.split_beam(c),
      '.' => {
        self.set(c, '|');
        0
      }
      _ => 0,
    }
  }

  fn split_beam(&mut self, src: &Coords) -> usize {
    src
      .sub_x(1)
      .to_coords()
      .map(|left| self.set(&left, '|'))
      .unwrap_or({});
    src
      .add_x(1)
      .to_coords()
      .map(|left| self.set(&left, '|'))
      .unwrap_or({});
    1
  }
}

#[cfg(test)]
mod tests {
  use crate::{day07::how_many_beam_splits, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      how_many_beam_splits(&mut read("./src/day07/sample.input")),
      21
    );
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      how_many_beam_splits(&mut read("./src/day07/my.input")),
      1690
    );
  }
}
