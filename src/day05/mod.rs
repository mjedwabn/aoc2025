use std::{cmp, io::BufRead};

use itertools::Itertools;

use crate::read_input;

type Long = u64;

pub fn how_many_ids_are_fresh(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let (ranges, ids) = parse_lines(lines);

  ids
    .iter()
    .filter(|id| ranges.iter().any(|r| r.in_range(id)))
    .count()
}

pub fn how_many_ids_are_fresh_according_to_fresh_ranges(input: &mut dyn BufRead) -> Long {
  let lines = read_input(input);
  let (ranges, _) = parse_lines(lines);

  join_overlapping_ranges(&ranges)
    .iter()
    .map(|r| r.end - r.start + 1)
    .sum()
}

fn parse_lines(lines: Vec<String>) -> (Vec<Range>, Vec<Long>) {
  fn parse_range(line: &String) -> Range {
    Range {
      start: line.split_once('-').unwrap().0.parse::<Long>().unwrap(),
      end: line.split_once('-').unwrap().1.parse::<Long>().unwrap(),
    }
  }

  fn parse_id(line: &String) -> Long {
    line.parse::<Long>().unwrap()
  }

  let mut parts = lines.split(|line| line == "");
  let ranges = parts
    .next()
    .unwrap()
    .iter()
    .map(|line| parse_range(line))
    .collect();
  let ids = parts
    .next()
    .unwrap()
    .iter()
    .map(|line| parse_id(line))
    .collect_vec();

  (ranges, ids)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Range {
  start: Long,
  end: Long,
}

impl Range {
  fn in_range(&self, id: &Long) -> bool {
    *id >= self.start && *id <= self.end
  }
}

fn join_overlapping_ranges(ranges: &Vec<Range>) -> Vec<Range> {
  let mut to_join: Vec<Range> = ranges.iter().map(|&r| r.clone()).collect_vec();

  loop {
    let joined = join(&to_join);

    if joined.len() == to_join.len() {
      return joined;
    } else {
      to_join = joined;
    }
  }
}

fn join(ranges: &Vec<Range>) -> Vec<Range> {
  let mut joined: Vec<Range> = Vec::new();

  for r in ranges {
    if let Some(to_join) = joined
      .clone()
      .iter()
      .find_position(|j| !(j.end + 2 <= r.start || j.start >= r.end + 2))
    {
      joined.remove(to_join.0);
      joined.push(Range {
        start: cmp::min(r.start, to_join.1.start),
        end: cmp::max(r.end, to_join.1.end),
      });
    } else {
      joined.push(*r);
    }
  }

  joined
}

#[cfg(test)]
mod tests {
  use crate::{
    day05::{how_many_ids_are_fresh, how_many_ids_are_fresh_according_to_fresh_ranges},
    read,
  };

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      how_many_ids_are_fresh(&mut read("./src/day05/sample.input")),
      3
    );
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      how_many_ids_are_fresh(&mut read("./src/day05/my.input")),
      789
    );
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(
      how_many_ids_are_fresh_according_to_fresh_ranges(&mut read("./src/day05/sample.input")),
      14
    );
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      how_many_ids_are_fresh_according_to_fresh_ranges(&mut read("./src/day05/my.input")),
      343329651880509
    );
  }
}
