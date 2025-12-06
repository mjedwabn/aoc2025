use std::io::BufRead;

use itertools::Itertools;

use crate::read_input;

type Long = u64;

pub fn answers_sum(input: &mut dyn BufRead) -> Long {
  let lines = read_input(input);
  let (numbers, operations) = (parse_numbers(&lines), parse_operations(&lines));

  let problems = operations.len();

  (0..problems)
    .map(|i| {
      solve_problem(
        &numbers.iter().map(|line| line[i]).collect_vec(),
        operations[i],
      )
    })
    .sum()
}

pub fn rtl_answers_sum(input: &mut dyn BufRead) -> Long {
  let lines = read_input(input);
  let (numbers, operations) = (parse_numbers_rtl(&lines), parse_operations(&lines));

  let problems = operations.len();

  (0..problems)
    .map(|i| solve_problem(&numbers[i], operations[i]))
    .sum()
}

fn parse_operations(lines: &Vec<String>) -> Vec<char> {
  lines[lines.len() - 1]
    .split_ascii_whitespace()
    .map(|o| o.chars().collect_vec()[0])
    .collect_vec()
}

fn parse_numbers(lines: &Vec<String>) -> Vec<Vec<Long>> {
  lines[0..lines.len() - 1]
    .iter()
    .map(|line| {
      line
        .split_ascii_whitespace()
        .map(|n| n.parse::<Long>().unwrap())
        .collect_vec()
    })
    .collect_vec()
}

fn parse_numbers_rtl(lines: &Vec<String>) -> Vec<Vec<Long>> {
  let to_transpose = lines[0..lines.len() - 1]
    .iter()
    .map(|s| s.chars().collect_vec())
    .collect_vec();
  let transposed_line_len = to_transpose.len();
  let line_len = lines[0].len();

  let transposed = (0..line_len)
    .map(|i| {
      (0..transposed_line_len)
        .map(|t| to_transpose[t][i])
        .collect::<String>()
    })
    .map(|s| s)
    .collect_vec();

  transposed
    .split(|t| t.trim() == "")
    .map(|p| {
      p.iter()
        .map(|n| n.trim().parse::<Long>().unwrap())
        .collect_vec()
    })
    .collect_vec()
}

fn solve_problem(numbers: &Vec<Long>, operation: char) -> Long {
  if operation == '+' {
    numbers.iter().sum()
  } else {
    numbers.iter().fold(1, |a, b| a * b)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    day06::{answers_sum, rtl_answers_sum},
    read,
  };

  #[test]
  fn sample_part1_input() {
    assert_eq!(answers_sum(&mut read("./src/day06/sample.input")), 4277556);
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      answers_sum(&mut read("./src/day06/my.input")),
      4771265398012
    );
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(
      rtl_answers_sum(&mut read("./src/day06/sample.input")),
      3263827
    );
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      rtl_answers_sum(&mut read("./src/day06/my.input")),
      10695785245101
    );
  }
}
