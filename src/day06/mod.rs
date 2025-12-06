use std::io::BufRead;

use itertools::Itertools;

use crate::read_input;

type Long = u64;

pub fn answers_sum(input: &mut dyn BufRead) -> Long {
  let lines = read_input(input);
  let (numbers, operations) = parse_lines(lines);

  let problems = operations.len();

  (0..problems).map(|i| {
    solve_problem(
      numbers.iter().map(|line| line[i]).collect_vec(),
      operations[i],
    )
  })
  .sum()
}

fn parse_lines(lines: Vec<String>) -> (Vec<Vec<Long>>, Vec<char>) {
  let nums = lines[0..lines.len() - 1]
    .iter()
    .map(|line| {
      line
        .split_ascii_whitespace()
        .map(|n| n.parse::<Long>().unwrap())
        .collect_vec()
    })
    .collect_vec();

  let ops = lines[lines.len() - 1]
    .split_ascii_whitespace()
    .map(|o| o.chars().collect_vec()[0])
    .collect_vec();

  (nums, ops)
}

fn solve_problem(numbers: Vec<Long>, operation: char) -> Long {
  if operation == '+' {
    numbers.iter().sum()
  }
  else {
    numbers.iter().fold(1, |a, b| a * b)
  }
}

#[cfg(test)]
mod tests {
  use crate::{day06::answers_sum, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(answers_sum(&mut read("./src/day06/sample.input")), 4277556);
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(answers_sum(&mut read("./src/day06/my.input")), 4771265398012);
  }
}
