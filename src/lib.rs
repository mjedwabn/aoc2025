use std::{fs::File, io::{BufRead, BufReader}};

pub mod day01;
pub mod day02;
pub mod day03;

pub fn read_input(input: &mut dyn BufRead) -> Vec<String> {
  input
    .lines()
    .map(|line| line.unwrap())
    .collect::<Vec<String>>()
}

pub fn read(file_name: &str) -> BufReader<File> {
  BufReader::new(File::open(file_name).unwrap())
}
