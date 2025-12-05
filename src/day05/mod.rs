use std::io::BufRead;

use itertools::Itertools;

use crate::read_input;

type Long = u64;

pub fn how_many_ids_are_fresh(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let (ranges, ids) = parse_lines(lines);

  ids.iter().filter(|id| ranges.iter().any(|r| r.in_range(id))).count()
}

fn parse_lines(lines: Vec<String>) -> (Vec<Range>, Vec<Long>) {
  fn parse_range(line: &String) -> Range {
    Range { 
      from: line.split_once('-').unwrap().0.parse::<Long>().unwrap(),
      to: line.split_once('-').unwrap().1.parse::<Long>().unwrap()
    }
  }

  fn parse_id(line: &String) -> Long {
    line.parse::<Long>().unwrap()
  }

  let mut parts = lines.split(|line| line == "");
  let ranges = parts.next().unwrap().iter().map(|line| parse_range(line)).collect();
  let ids = parts.next().unwrap().iter().map(|line| parse_id(line)).collect_vec();

  (ranges, ids)
}

struct Range {
  from: Long,
  to: Long
}

impl Range {
  fn in_range(&self, id: &Long) -> bool {
    *id >= self.from && *id <= self.to
  }
}

#[cfg(test)]
mod tests {
    use crate::{day05::{how_many_ids_are_fresh}, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(how_many_ids_are_fresh(&mut read("./src/day05/sample.input")), 3);
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(how_many_ids_are_fresh(&mut read("./src/day05/my.input")), 789);
  }
}