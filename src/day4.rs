type Position = i32;

#[derive(Clone,Debug)]
pub struct PrintShop {
  rows: Vec<Vec<bool>>,
  width: Position,
}

impl PrintShop {
  fn is_empty(&self, x: Position, y: Position) -> bool {
    x < 0 || y < 0 || x >= self.width || y >= self.rows.len() as Position ||
        !self.rows[y as usize][x as usize]
  }

  fn remove(&mut self, x: Position, y: Position) {
    self.rows[y as usize][x as usize] = false;
  }

  fn is_moveable(&self, x: Position, y: Position) -> bool {
    if self.is_empty(x, y) {
      false
    } else {
      let mut result = 0;
      for delta_x in -1..=1 {
        for delta_y in -1..=1 {
          if (delta_x != 0 || delta_y != 0) && !self.is_empty(x + delta_x, y + delta_y) {
            result += 1;
          }
        }
      }
      result < 4
    }
  }
}

fn parse_line(line: &str) -> Result<Vec<bool>, String> {
  line.chars().map(|c| match c {
                               '@' => Ok(true),
                               '.' => Ok(false),
                               _ => Err(format!("Invalid character: {c}")),
                             })
      .collect::<Result<Vec<bool>,String>>()
}

pub fn generator(input: &str) -> PrintShop {
  let rows = input.lines().map(parse_line)
      .collect::<Result<Vec<Vec<bool>>,String>>()
      .expect("Can't parse input");
  let width = rows.iter().map(|r| r.len()).max().unwrap() as Position;
  PrintShop{rows, width}
}

pub fn part1(input: &PrintShop) -> usize {
  let mut result = 0;
  for y in 0..(input.rows.len() as Position) {
    for x in 0..input.width {
      if input.is_moveable(x, y) {
        result += 1;
      }
    }
  }
  result
}

fn find_removeable(input: &PrintShop) -> Vec<(Position, Position)> {
  let mut result = Vec::new();
  for y in 0..(input.rows.len() as Position) {
    for x in 0..input.width {
      if input.is_moveable(x, y) {
        result.push((x,y));
      }
    }
  }
  result
}

pub fn part2(input: &PrintShop) -> usize {
  let mut input = input.clone();
  let mut result = 0;
  loop {
    let to_remove = find_removeable(&input);
    if to_remove.is_empty() { break; }
    result += to_remove.len();
    for (x, y) in to_remove {
      input.remove(x, y);
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(13, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(43, part2(&data));
  }
}
