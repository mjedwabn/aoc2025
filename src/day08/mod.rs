use std::{collections::HashSet, hash::Hash, io::BufRead};

use itertools::Itertools;

use crate::read_input;

pub fn multiplied_three_largest_circuits(input: &mut dyn BufRead, n: usize) -> usize {
  let lines = read_input(input);
  let boxes = parse_junction_boxes(&lines);
  let connections = make_connections(&boxes);
  let available_connections = &connections[..n];
  let circuits = make_circuits(available_connections, vec![]);
  let merged_circuits = merge_circuits(circuits);

  merged_circuits
    .iter()
    .sorted_by_key(|c| c.len())
    .rev()
    .take(3)
    .fold(1, |acc, n| acc * n.len())
}

pub fn multipied_x_coords_of_last_two_junction_boxes(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let boxes = parse_junction_boxes(&lines);
  let connections = make_connections(&boxes);
  let mut circuits = boxes.iter().map(|b| Circuit::from([*b])).collect_vec();

  for c in connections {
    circuits = make_circuits(&[c.clone()], circuits);
    circuits = merge_circuits(circuits);

    if circuits.len() == 1 {
      return c.a.x * c.b.x;
    }
  }
  0
}

fn make_connections(boxes: &Vec<JunctionBox>) -> Vec<Connection> {
  boxes
    .iter()
    .enumerate()
    .flat_map(|(ai, a)| {
      boxes
        .iter()
        .enumerate()
        .filter(move |(bi, _)| *bi > ai)
        .map(|(_, &b)| {
          (
            (Connection { a: *a, b }),
            (((a.x as isize - b.x as isize).abs().pow(2)
              + (a.y as isize - b.y as isize).abs().pow(2)
              + (a.z as isize - b.z as isize).abs().pow(2)) as f32)
              .sqrt(),
          )
        })
    })
    .sorted_by(|a, b| a.1.total_cmp(&b.1))
    .map(|c| c.0)
    .collect_vec()
}

fn make_circuits(connections: &[Connection], initial_circuits: Vec<Circuit>) -> Vec<Circuit> {
  let mut circuits: Vec<Circuit> = initial_circuits;

  for conn in connections.iter() {
    if let Some(circuit) = circuits
      .iter_mut()
      .filter(|c| !(c.contains(&conn.a) && c.contains(&conn.b)))
      .find(|c| c.contains(&conn.a) || c.contains(&conn.b))
    {
      circuit.insert(conn.a);
      circuit.insert(conn.b);
    } else {
      circuits.push(Circuit::from([conn.a, conn.b]));
    }
  }

  circuits
}

fn merge_circuits(circuits: Vec<Circuit>) -> Vec<Circuit> {
  let mut merged = true;
  let mut merged_circuits = circuits;

  while merged {
    let before = merged_circuits.len();
    merged_circuits = merge_once(merged_circuits);
    merged = before != merged_circuits.len();
  }

  merged_circuits
}

fn merge_once(circuits: Vec<Circuit>) -> Vec<Circuit> {
  let mut merged = circuits;
  let mut to_remove: HashSet<usize> = HashSet::new();

  for c1 in 0..merged.len() {
    'inner: for c2 in c1 + 1..merged.len() {
      if to_remove.contains(&c2) {
        continue;
      }
      for j1 in merged[c1].iter() {
        if merged[c2].contains(&j1) {
          let circuit = merged[c2].clone();
          for j in circuit {
            merged[c1].insert(j);
            to_remove.insert(c2);
          }
          continue 'inner;
        }
      }
    }
  }

  for r in to_remove.iter().sorted().rev() {
    merged.remove(*r);
  }

  merged
}

fn parse_junction_boxes(lines: &Vec<String>) -> Vec<JunctionBox> {
  lines
    .iter()
    .map(|line| line.split(','))
    .map(|mut parts| JunctionBox {
      x: parts.next().unwrap().parse::<usize>().unwrap(),
      y: parts.next().unwrap().parse::<usize>().unwrap(),
      z: parts.next().unwrap().parse::<usize>().unwrap(),
    })
    .collect_vec()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct JunctionBox {
  x: usize,
  y: usize,
  z: usize,
}

#[derive(Clone, Copy)]
struct Connection {
  a: JunctionBox,
  b: JunctionBox,
}

type Circuit = HashSet<JunctionBox>;

#[cfg(test)]
mod tests {
  use crate::{
    day08::{multipied_x_coords_of_last_two_junction_boxes, multiplied_three_largest_circuits},
    read,
  };

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      multiplied_three_largest_circuits(&mut read("./src/day08/sample.input"), 10),
      40
    );
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      multiplied_three_largest_circuits(&mut read("./src/day08/my.input"), 1000),
      244188
    );
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(
      multipied_x_coords_of_last_two_junction_boxes(&mut read("./src/day08/sample.input")),
      25272
    );
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      multipied_x_coords_of_last_two_junction_boxes(&mut read("./src/day08/my.input")),
      8361881885
    );
  }
}
