use std::str::Chars;
use std::vec::Vec;

type Number = u64;

#[derive(Debug)]
enum Operation {Add, Multiply}

#[derive(Debug)]
pub struct NumberTable<'a> {
  lines: Vec<&'a str>,
  operations: Vec<Operation>,
}

fn parse_int(s: &str) -> Result<Number, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

fn parse_operator(s: &str) -> Result<Operation, String> {
  match s {
    "+" => Ok(Operation::Add),
    "*" => Ok(Operation::Multiply),
    _ => Err(format!("Unknown operator: '{s}'"))
  }
}

pub fn generator(input: & str) -> NumberTable<'_> {
  let mut lines: Vec<&str> = input.lines().collect();
  let operations = lines.pop().unwrap().split_whitespace()
      .map(parse_operator)
      .collect::<Result<Vec<Operation>, String>>()
      .expect("Bad operations");
  NumberTable{lines, operations}
}

fn calculate_problems(nums: &[Vec<Number>], ops: &[Operation]) -> Number {
  nums.iter().zip(ops.iter())
      .map(|(row, op)| match op {
        Operation::Add => row.iter().sum::<Number>(),
        Operation::Multiply => row.iter().product::<Number>(),
      } ).sum()
}

fn parse_part1(table: &NumberTable) -> Vec<Vec<Number>> {
  let mut result =
      vec![Vec::with_capacity(table.lines.len()); table.operations.len()];
  for &line in &table.lines {
    for (i, word) in line.split_whitespace().enumerate() {
      result[i].push(parse_int(word).expect("Bad data"));
    }
  }
  result
}

pub fn part1(input: &NumberTable) -> Number {
  calculate_problems(&parse_part1(input), &input.operations)
}

/// Parse the data vertically for part 2.
fn parse_part2(table: &NumberTable) -> Vec<Vec<Number>> {
  // Build each line from a column of numbers. Move to a problem when we find
  // a blank line.
  // Create a chars iterator for each of the data lines.
  let mut iters: Vec<Chars<'_>> = table.lines.iter()
      .map(|line| line.chars()).collect();
  // The column that we are appending to.
  let mut column = 0;
  let mut result =
      vec![Vec::with_capacity(table.lines.len()); table.operations.len()];
  // build the string using one character from each data line.
  while let Some(line) = iters.iter_mut()
      .map(|itr| itr.next())
      .collect::<Option<String>>() {
    // if the line is empty, we go to the next problem.
    if line.trim().is_empty() {
      column += 1;
    } else {
      result[column].push(parse_int(line.trim()).expect("Can't parse integer - '{line}'"));
    }
  }
  result
}

pub fn part2(input: &NumberTable) -> Number {
  calculate_problems(&parse_part2(input), &input.operations)
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
