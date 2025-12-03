#[derive(Debug)]
pub struct BatteryShelf {
  rows: Vec<Vec<u64>>,
}

impl BatteryShelf {
  fn find_max(row: &[u64], count: usize) -> u64 {
    assert!(count <= row.len(), "count is bigger than row");
    let mut remaining = count;
    let mut result = 0;
    let mut current = 0;
    while remaining > 0 {
      let best = row[current..=(row.len() - remaining)].iter().max().unwrap();
      result = result * 10 + best;
      current = row[current..].iter().position(|&x| x == *best).unwrap() + current + 1;
      remaining -= 1;
    }
    result
  }
}

fn parse_line(line: &str) -> Result<Vec<u64>, String> {
  line.chars().map(|c| c.to_digit(10)
        .map(|c| c as u64)
        .ok_or_else(|| format!("Can't parse character '{c}'")))
      .collect::<Result<Vec<u64>,String>>()
}

pub fn generator(input: &str) -> BatteryShelf {
  let rows = input.lines().map(parse_line)
      .collect::<Result<Vec<Vec<u64>>,String>>()
      .expect("Can't parse input");
  BatteryShelf{rows}
}

pub fn part1(input: &BatteryShelf) -> u64 {
  input.rows.iter().map(|row| BatteryShelf::find_max(row, 2)).sum()
}

pub fn part2(input: &BatteryShelf) -> u64 {
  input.rows.iter().map(|row| BatteryShelf::find_max(row, 12)).sum()
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
