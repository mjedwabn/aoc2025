use std::{collections::HashSet, io::BufRead};

use crate::{RemoveFirst, read_input};

fn fewest_button_presses(input: &mut dyn BufRead) -> usize {
  let lines = read_input(input);
  let machines = parse_machines(&lines);

  machines.iter().map(|m| m.fewest_button_presses()).sum()
}

fn parse_machines(lines: &Vec<String>) -> Vec<Machine> {
  lines.iter().map(|l| parse_machine(l)).collect()
}

fn parse_machine(line: &String) -> Machine {
  let re =
    regex::Regex::new(r"\[(?<lights>[\.#]+)\](?<buttons>( \([0-9,]+\))+) \{(?<joltages>[0-9,]+)\}")
      .unwrap();
  let captures = re.captures(line).unwrap();
  let indicator_light_diagram: Vec<bool> = captures["lights"].chars().map(|c| c == '#').collect();
  let button_wiring_schematics: Vec<Vec<u32>> = captures["buttons"]
    .trim()
    .split(' ')
    .map(|b| {
      let nums: Vec<u32> = b[1..b.len() - 1]
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
      nums
    })
    .collect();
  let joltage_requirements: Vec<u32> = captures["joltages"]
    .split(',')
    .map(|n| n.parse::<u32>().unwrap())
    .collect();

  let n = indicator_light_diagram.len() as u32;

  Machine {
    indicator_light_diagram: indicator_light_diagram
      .iter()
      .fold(0u16, |acc, &b| (acc << 1) | if b { 1 } else { 0 }),
    button_wiring_schematics: button_wiring_schematics
      .iter()
      .map(|btn| btn.iter().fold(0u16, |acc, &b| acc + 2_u16.pow(n - b - 1)))
      .collect(),
    joltage_requirements,
  }
}

struct Machine {
  indicator_light_diagram: u16,
  button_wiring_schematics: Vec<u16>,
  joltage_requirements: Vec<u32>,
}

impl Machine {
  fn fewest_button_presses(&self) -> usize {
    let mut visited_lights = HashSet::<u16>::new();
    let initial_lights = 0u16;
    let mut queue: Vec<(u16, usize)> = vec![(initial_lights, 0)];

    while let Some((current_lights, presses)) = queue.remove_first() {
      if current_lights == self.indicator_light_diagram {
        return presses;
      }

      for &button in &self.button_wiring_schematics {
        let new_lights = current_lights ^ button;

        if !visited_lights.contains(&new_lights) {
          visited_lights.insert(new_lights);
          queue.push((new_lights, presses + 1));
        }
      }
    }

    panic!("No solution found");
  }
}

#[cfg(test)]
mod tests {
  use crate::{day10::fewest_button_presses, read};

  #[test]
  fn sample_part1_input() {
    assert_eq!(
      fewest_button_presses(&mut read("./src/day10/sample.input")),
      7
    );
  }

  #[test]
  fn my_part1_input() {
    assert_eq!(
      fewest_button_presses(&mut read("./src/day10/my.input")),
      461
    );
  }
}
