use std::cmp::Ordering;
use itertools::Itertools;

fn parse_int(s: &str) -> Result<i64, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

fn parse_line(s: &str) -> Result<i64, String> {
  let (direct,size) = s.split_at(1);
  let mut size = parse_int(size)?;
  if direct == "L" {
    size = -size;
  }
  Ok(size)
}

pub fn generator(input: &str) -> Vec<i64> {
  input.lines().map(parse_line).try_collect().expect("Can't parse input")
}

const START: i64 = 50;
const SIZE: i64 = 100;

pub fn part1(input: &[i64]) -> usize {
  let mut current = START;
  let mut result = 0;
  for turn in input {
    current = (current + turn).rem_euclid(SIZE);
    if current == 0 {
      result += 1;
    }
  }
  result
}

pub fn part2(input: &[i64]) -> usize {
  let mut current = START;
  let mut result = 0;
  for turn in input {
    match turn.cmp(&0) {
      Ordering::Less =>
        result += ((-current).rem_euclid(SIZE) - turn).div_euclid(SIZE) as usize,
      Ordering::Greater =>
        result += (current + turn).div_euclid(SIZE) as usize,
      _ => {}
    }
    current = (current + turn).rem_euclid(SIZE);
  }
  result
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(3, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(6, part2(&data));
  }
}
