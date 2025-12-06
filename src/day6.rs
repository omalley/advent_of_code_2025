use std::str::Chars;
use std::vec::Vec;

type Number = u64;

#[derive(Debug)]
enum Operation {ADD, MULTIPLY}

#[derive(Debug)]
pub struct NumberTable {
  data: Vec<Vec<Number>>,
  vertical: Vec<Vec<Number>>,
  operations: Vec<Operation>,
}

fn parse_int(s: &str) -> Result<Number, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

fn parse_operator(s: &str) -> Result<Operation, String> {
  match s.trim() {
    "+" => Ok(Operation::ADD),
    "*" => Ok(Operation::MULTIPLY),
    _ => Err(format!("Unknown operator: '{s}'"))
  }
}

pub fn generator(input: &str) -> NumberTable {
  let mut lines: Vec<&str> = input.lines().collect();
  let operations = lines.pop().unwrap().split_whitespace()
      .map(parse_operator)
      .collect::<Result<Vec<Operation>,String>>()
      .expect("Bad operations");
  let mut data = vec![Vec::new(); operations.len()];
  for line in &lines {
    for (i, word) in line.split_whitespace().enumerate() {
      data[i].push(parse_int(word).expect("Bad data"));
    }
  }
  // Parse the data vertically for part 2.
  // Build each line from a column of numbers. Move to a problem when we find
  // a blank line.
  // Create a chars iterator for each of the data lines.
  let mut iters: Vec<Chars<'_>> = lines.iter()
      .map(|line| line.chars()).collect();
  // The column that we are appending to.
  let mut column = 0;
  let mut vertical = vec![Vec::new(); operations.len()];
  while let Some(line) = iters.iter_mut()
      .map(|itr| itr.next())
      .collect::<Option<String>>() {
    if line.trim().is_empty() {
      column += 1;
    } else {
      vertical[column].push(parse_int(line.trim()).expect("Can't parse integer - '{line}'"));
    }
  }
  NumberTable { data, vertical, operations }
}

pub fn part1(input: &NumberTable) -> Number {
  input.data.iter().zip(input.operations.iter())
      .map(|(row, op)| match op {
        Operation::ADD => row.iter().sum::<Number>(),
        Operation::MULTIPLY => row.iter().product::<Number>(),
      } ).sum()
}

pub fn part2(input: &NumberTable) -> Number {
  input.vertical.iter().zip(input.operations.iter())
      .map(|(row, op)| match op {
        Operation::ADD => row.iter().sum::<Number>(),
        Operation::MULTIPLY => row.iter().product::<Number>(),
      } ).sum()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +\n";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(4277556, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(3263827, part2(&data));
  }
}
