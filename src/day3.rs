use itertools::Itertools;

#[derive(Debug)]
pub struct BatteryShelf {
  rows: Vec<Vec<u64>>,
}

impl BatteryShelf {
  fn find_max(row: &[u64], count: usize) -> u64 {
    row.iter().combinations(count)
        .map(|win| win.iter().fold(0, |acc, x| acc * 10 + *x))
        .max()
        .expect("Can't find max")
  }
}

fn parse_line(line: &str) -> Result<Vec<u64>, String> {
  Ok(line.chars().map(|c| c.to_digit(10)
        .map(|c| c as u64)
        .ok_or_else(|| format!("Can't parse character '{c}'")))
      .collect::<Result<Vec<u64>,String>>()?)
}

pub fn generator(input: &str) -> BatteryShelf {
  let rows = input.lines().map(parse_line)
      .collect::<Result<Vec<Vec<u64>>,String>>()
      .expect("Can't parse input");
  BatteryShelf{rows}
}

pub fn part1(input: &BatteryShelf) -> u64 {
  input.rows.iter().map(|row| BatteryShelf::find_max(&row, 2)).sum()
}

pub fn part2(input: &BatteryShelf) -> u64 {
  input.rows.iter().map(|row| BatteryShelf::find_max(&row, 12)).sum()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"987654321111111
811111111111119
234234234234278
818181911112111";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(357, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(3121910778619, part2(&data));
  }
}
