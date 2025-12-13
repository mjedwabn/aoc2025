use std::{collections::HashSet, hash::Hash, io::BufRead};

use itertools::Itertools;

use crate::read_input;

pub fn multiplied_three_largest_circuits(input: &mut dyn BufRead, n: usize) -> usize {
  let lines = read_input(input);
  let boxes = parse_junction_boxes(&lines);

  let connections = boxes
    .iter()
    .enumerate()
    .flat_map(|(bi, b)| {
      let m = boxes
        .iter()
        .enumerate()
        .filter(move |(bbi, _)| *bbi > bi)
        .map(|(_, &bb)| {
          (
            (Connection { a: *b, b: bb }),
            (((b.x as isize - bb.x as isize).abs().pow(2)
              + (b.y as isize - bb.y as isize).abs().pow(2)
              + (b.z as isize - bb.z as isize).abs().pow(2)) as f32)
              .sqrt(),
          )
        });
      m
    })
    .sorted_by(|a, b| a.1.total_cmp(&b.1))
    .collect_vec();

  let circuits = make_circuits(&connections, n);

  let mut merged = true;
  let mut merged_circuits = circuits;

  while merged {
    let before = merged_circuits.len();
    merged_circuits = merge_circuits(&merged_circuits);
    merged = before != merged_circuits.len();
  }

  let lonely_boxes = boxes
    .iter()
    .filter(|b| !merged_circuits.iter().any(|c| c.contains(&b)))
    .collect_vec();

  for b in lonely_boxes {
    let mut circuit = HashSet::new();
    circuit.insert(*b);
    merged_circuits.push(circuit);
  }

  merged_circuits
    .iter()
    .sorted_by_key(|c| c.len())
    .rev()
    .take(3)
    .fold(1, |acc, n| acc * n.len())
}

fn make_circuits(connections: &Vec<(Connection, f32)>, n: usize) -> Vec<HashSet<JunctionBox>> {
  let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();
  let available_connections = &connections[..n];

  for (conn, _) in available_connections.iter() {
    if let Some(circuit) = circuits
      .iter_mut()
      .filter(|c| !(c.contains(&conn.a) && c.contains(&conn.b)))
      .find(|c| c.contains(&conn.a) || c.contains(&conn.b))
    {
      circuit.insert(conn.a);
      circuit.insert(conn.b);
    } else {
      let mut circuit: HashSet<JunctionBox> = HashSet::new();
      circuit.insert(conn.a);
      circuit.insert(conn.b);
      circuits.push(circuit);
    }
  }

  circuits
}

fn merge_circuits(circuits: &Vec<HashSet<JunctionBox>>) -> Vec<HashSet<JunctionBox>> {
  let mut merged = circuits.clone();
  let mut to_remove: Vec<usize> = vec![];

  for c1 in 0..merged.len() {
    for c2 in 0..merged.len() {
      if c2 > c1 {
        for j1 in merged[c1].clone() {
          if merged[c2].contains(&j1) {
            let circuit = merged[c2].clone();
            for j in circuit {
              merged[c1].insert(j);
              to_remove.push(c2);
            }
            continue;
          }
        }
      }
    }
  }

  merged
    .iter()
    .enumerate()
    .filter(|(i, _)| !to_remove.contains(i))
    .map(|(_, c)| c.clone())
    .collect_vec()
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

struct Connection {
  a: JunctionBox,
  b: JunctionBox,
}

#[cfg(test)]
mod tests {
  use crate::{day08::multiplied_three_largest_circuits, read};

  #[test]
  fn sample_part1_part() {
    assert_eq!(
      multiplied_three_largest_circuits(&mut read("./src/day08/sample.input"), 10),
      40
    );
  }

  #[test]
  fn my_part1_part() {
    assert_eq!(
      multiplied_three_largest_circuits(&mut read("./src/day08/my.input"), 1000),
      244188
    );
  }
}
