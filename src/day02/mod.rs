use std::io::BufRead;

use crate::read_input;

type Long = u64;

pub fn sum_invalid_ids(input: &mut dyn BufRead) -> Long {
  let ranges = parse_ranges(read_input(input).get(0).unwrap());
  ranges
    .iter()
    .map(|range| sum_invalid_ids_in_range(range))
    .sum()
}

fn sum_invalid_ids_in_range(range: &(Long, Long)) -> Long {
  (range.0..=range.1).filter(|n| is_invalid_id(*n)).sum()
}

fn is_invalid_id(id: Long) -> bool {
  let s = id.to_string();
  if s.len() % 2 != 0 {
    false
  } else {
    s[0..s.len() / 2] == s[s.len() / 2..s.len()]
  }
}

fn parse_ranges(input: &str) -> Vec<(Long, Long)> {
  input
    .split(',')
    .map(|r| {
      let mut bounds = r.split('-').map(|b| b.parse::<Long>().unwrap());
      (bounds.next().unwrap(), bounds.next().unwrap())
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use crate::{day02::sum_invalid_ids, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      sum_invalid_ids(&mut read("./src/day02/sample.input")),
      1227775554
    );
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      sum_invalid_ids(&mut read("./src/day02/my.input")),
      16793817782
    );
  }
}
