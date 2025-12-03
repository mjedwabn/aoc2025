use std::io::BufRead;

use crate::read_input;

pub fn total_output_joltage(input: &mut dyn BufRead) -> i32 {
  let banks = parse_banks(read_input(input));
  banks.iter().map(|bank| find_largest_joltage(bank)).sum()
}

fn parse_banks(lines: Vec<String>) -> Vec<Vec<i32>> {
  lines
    .iter()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
    })
    .collect()
}

fn find_largest_joltage(bank: &Vec<i32>) -> i32 {
  let mut max_joltage = 0;
  (0..bank.len() - 1).for_each(|i| {
    bank[i + 1..bank.len()].iter().for_each(|b| {
      let a = bank[i as usize];
      let joltage = a * 10 + b;
      if joltage > max_joltage {
        max_joltage = joltage;
      }
    });
  });

  max_joltage
}

#[cfg(test)]
mod tests {
  use crate::{day03::total_output_joltage, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      total_output_joltage(&mut read("./src/day03/sample.input")),
      357
    );
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      total_output_joltage(&mut read("./src/day03/my.input")),
      17229
    );
  }
}
