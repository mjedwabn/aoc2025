use std::io::BufRead;

use crate::read_input;

type Long = u64;

pub fn total_output_joltage(input: &mut dyn BufRead, digits: usize) -> Long {
  let banks = parse_banks(read_input(input));
  banks
    .iter()
    .map(|bank| find_max_joltage(bank, digits, 0, 0))
    .sum()
}

fn parse_banks(lines: Vec<String>) -> Vec<Vec<u32>> {
  lines
    .iter()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u32)
        .collect()
    })
    .collect()
}

fn find_max_joltage(bank: &Vec<u32>, digits: usize, digit: usize, start: usize) -> Long {
  let max = bank[start..=bank.len() - digits + digit]
    .iter()
    .max()
    .unwrap();
  let index = bank
    .iter()
    .enumerate()
    .position(|(i, n)| i >= start && n == max)
    .unwrap();

  *max as Long * 10u64.pow((digits as i32 - digit as i32 - 1) as u32)
    + if digit + 1 == digits {
      0
    } else {
      find_max_joltage(bank, digits, digit + 1, index + 1)
    }
}

#[cfg(test)]
mod tests {
  use crate::{day03::total_output_joltage, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      total_output_joltage(&mut read("./src/day03/sample.input"), 2),
      357
    );
  }

  #[test]
  fn sample_part1_input_generic() {
    assert_eq!(
      total_output_joltage(&mut read("./src/day03/sample.input"), 2),
      357
    );
  }

  #[test]
  fn sample_part2_input() {
    assert_eq!(
      total_output_joltage(&mut read("./src/day03/sample.input"), 12),
      3121910778619
    );
  }

  #[test]
  fn my_part2_input() {
    assert_eq!(
      total_output_joltage(&mut read("./src/day03/my.input"), 12),
      170520923035051
    );
  }
}
