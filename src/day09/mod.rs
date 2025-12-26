use std::{cmp, io::BufRead};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::read_input;

pub fn area_of_largest_rectangle(input: &mut dyn BufRead) -> usize {
  let polygon = read_polygon(input);

  make_unique_pairs(&polygon)
    .iter()
    .map(|pair| area(pair.0, pair.1))
    .max()
    .unwrap()
}

pub fn area_of_largest_red_green_rectangle(input: &mut dyn BufRead) -> usize {
  let coords = read_polygon(input);

  make_unique_pairs(&coords)
    .par_iter()
    .filter(|p| is_rectangle_red_green(p, &coords))
    .map(|p| p)
    .map(|pair| area(pair.0, pair.1))
    .max()
    .unwrap()
}

fn read_polygon(input: &mut dyn BufRead) -> Vec<Coords> {
  let lines = read_input(input);
  parse_coords(lines)
}

fn parse_coords(lines: Vec<String>) -> Vec<Coords> {
  lines
    .iter()
    .map(|line| line.split_once(','))
    .map(|split| Coords {
      x: split.unwrap().0.parse().unwrap(),
      y: split.unwrap().1.parse().unwrap(),
    })
    .collect_vec()
}

fn make_unique_pairs(coords: &Vec<Coords>) -> Vec<(Coords, Coords)> {
  coords
    .iter()
    .enumerate()
    .flat_map(|c1| {
      coords
        .iter()
        .enumerate()
        .filter(move |c2| c2.0 > c1.0)
        .map(move |c2| (*c1.1, *c2.1))
    })
    .collect_vec()
}

fn area(a: Coords, b: Coords) -> usize {
  (1 + cmp::max(a.x, b.x) - cmp::min(a.x, b.x)) * (1 + cmp::max(a.y, b.y) - cmp::min(a.y, b.y))
}

fn is_rectangle_red_green(c: &(Coords, Coords), coords: &Vec<Coords>) -> bool {
  let c1 = c.0;
  let c3 = c.1;

  let c2 = Coords { x: c1.x, y: c3.y };
  let c4 = Coords { x: c3.x, y: c1.y };

  let vertices_red_green = vec![c2, c4].iter().all(|s| is_inside_polygon(s, coords));

  let edges_red_green = vertices_red_green
    && vec![(c1, c2), (c2, c3), (c3, c4), (c4, c1)]
      .iter()
      .all(|(start, end)| {
        coords_between(start, end)
          .iter()
          .all(|p| is_inside_polygon(&p, coords))
      });

  if edges_red_green {
    println!(
      "Found red-green rectangle: {:?} to {:?} = {}",
      c2,
      c4,
      area(c2, c4)
    );
  }

  edges_red_green
}

fn is_inside_polygon(point: &Coords, polygon: &Vec<Coords>) -> bool {
  let mut intersections = 0;
  let n = polygon.len();

  for i in 0..n {
    let p1 = &polygon[i];
    let p2 = &polygon[(i + 1) % n];

    if is_on_edge(point, &p1, &p2) {
      return true;
    }

    if intersects_edge(point, p1, p2) {
      intersections += 1;
    }
  }

  intersections % 2 == 1
}

fn is_on_edge(point: &Coords, p1: &Coords, p2: &Coords) -> bool {
  let Coords { x, y } = *point;

  (p1.x == p2.x && x == p1.x && ((p1.y <= y && y <= p2.y) || (p2.y <= y && y <= p1.y)))
    || (p1.y == p2.y && y == p1.y && ((p1.x <= x && x <= p2.x) || (p2.x <= x && x <= p1.x)))
}

fn intersects_edge(point: &Coords, p1: &Coords, p2: &Coords) -> bool {
  let Coords { x, y } = *point;

  y > cmp::min(p1.y, p2.y) && y <= cmp::max(p1.y, p2.y) && x < cmp::max(p1.x, p2.x) && p1.x == p2.x
}

fn coords_between(start: &Coords, end: &Coords) -> Vec<Coords> {
  (cmp::min(start.y, end.y)..=cmp::max(start.y, end.y))
    .flat_map(|y| {
      (cmp::min(start.x, end.x)..=cmp::max(start.x, end.x)).map(move |x| Coords { x, y })
    })
    .collect()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Coords {
  x: usize,
  y: usize,
}

#[cfg(test)]
mod tests {
  use crate::{
    day09::{
      Coords, area_of_largest_rectangle, area_of_largest_red_green_rectangle, is_inside_polygon,
    },
    read,
  };

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

  #[test]
  fn sample_part2_input() {
    assert_eq!(
      area_of_largest_red_green_rectangle(&mut read("./src/day09/sample.input")),
      24
    );
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      area_of_largest_red_green_rectangle(&mut read("./src/day09/my.input")),
      1644094530
    );
  }

  #[test]
  fn inside_polygon() {
    assert_eq!(
      is_inside_polygon(
        &Coords { x: 3, y: 2 },
        &vec![
          Coords { x: 2, y: 0 },
          Coords { x: 5, y: 0 },
          Coords { x: 5, y: 2 },
          Coords { x: 7, y: 2 },
          Coords { x: 7, y: 4 },
          Coords { x: 2, y: 4 },
        ]
      ),
      true
    );
  }
}
