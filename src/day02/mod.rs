use std::io::BufRead;

use itertools::Itertools;

use crate::read_input;

type Long = u64;

pub fn sum_invalid_ids(input: &mut dyn BufRead) -> Long {
  let ranges = parse_ranges(read_input(input).get(0).unwrap());
  ranges
    .iter()
    .map(|range| sum_invalid_ids_in_range(range, is_id_made_of_two_repeated_parts))
    .sum()
}

pub fn sum_invalid_ids_part2(input: &mut dyn BufRead) -> Long {
  let ranges = parse_ranges(read_input(input).get(0).unwrap());
  ranges
    .iter()
    .map(|range| sum_invalid_ids_in_range(range, is_id_made_of_at_least_two_repeated_parts))
    .sum()
}

fn sum_invalid_ids_in_range(range: &(Long, Long), validator: fn(Long) -> bool) -> Long {
  (range.0..=range.1).filter(|n| validator(*n)).sum()
}

fn is_id_made_of_two_repeated_parts(id: Long) -> bool {
  let s = id.to_string();
  if s.len() % 2 != 0 {
    false
  } else {
    s[0..s.len() / 2] == s[s.len() / 2..s.len()]
  }
}

fn is_id_made_of_at_least_two_repeated_parts(id: Long) -> bool {
  let s = id.to_string();

  find_divisors(s.len()).iter().any(|&d| {
    let parts = s.len() / d;
    let part = &s[0..d];
    (1..parts).all(|i| &s[i * d..(i + 1) * d] == part)
  })
}

fn find_divisors(n: usize) -> Vec<usize> {
  (1..=n / 2).filter(|i| n % i == 0).rev().collect_vec()
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
  use crate::{
    day02::{sum_invalid_ids, sum_invalid_ids_part2},
    read,
  };

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

  #[test]
  fn sample_part2_input() {
    assert_eq!(
      sum_invalid_ids_part2(&mut read("./src/day02/sample.input")),
      4174379265
    );
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      sum_invalid_ids_part2(&mut read("./src/day02/my.input")),
      27469417404
    );
  }
}
