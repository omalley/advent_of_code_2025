use smallvec::SmallVec;

type RollId = usize;
type NeighborVec = SmallVec<[RollId; 8]>;

#[derive(Debug)]
pub struct Roll {
  neighbors: NeighborVec,
}

type IdGrid = Vec<Vec<Option<RollId>>>;

#[derive(Debug)]
pub struct PrintShop {
  rolls: Vec<Roll>,
}

impl PrintShop {
  const MIN_NEIGHBORS: usize = 4;

  /// Parse a grid location
  fn parse_char(ch: char) -> Result<bool, String> {
    match ch {
      '@' => Ok(true),
      '.' => Ok(false),
      _ => Err(format!("Invalid character: {ch}")),
    }
  }

  /// Read the input and return a grid of the roll id at each
  /// location and the number of roll ids.
  fn assign_ids(input: &str) -> Result<(IdGrid, usize), String> {
    let mut next_id = 0;
    let result = input.lines().map(|line| {
      line.chars().map(|ch| if Self::parse_char(ch)? {
        next_id += 1;
        Ok(Some(next_id - 1))
      } else {
        Ok(None)
      }).collect::<Result<Vec<Option<usize>>, String>>()
    }).collect::<Result<Vec<Vec<Option<usize>>>, String>>()?;
    Ok((result, next_id))
  }

  fn add_neighbors(&mut self, left: RollId, right: RollId) {
    self.rolls[left].neighbors.push(right);
    self.rolls[right].neighbors.push(left);
  }

  fn find_moveable(&self) -> Vec<RollId> {
    self.rolls.iter().enumerate()
        .filter(|(_, roll)| roll.neighbors.len() < Self::MIN_NEIGHBORS)
        .map(|(i, _)| i)
        .collect()
  }
}

pub fn generator(input: &str) -> PrintShop {
  let (id_grid, max_id) =
      PrintShop::assign_ids(input).expect("Invalid input");
  let width = id_grid[0].len();
  let mut result = PrintShop{rolls: Vec::with_capacity(max_id)};
  for (y, row) in id_grid.iter().enumerate() {
    for (x, loc) in row.iter().enumerate() {
      if let Some(id) = loc {
        result.rolls.push(Roll{neighbors: NeighborVec::new()});
        if y != 0 {
          for prev in
              id_grid[y -1][(x.max(1)-1)..(x+2).min(width)].iter().flatten() {
            result.add_neighbors(*prev, *id);
          }
        }
        if x != 0 && let Some(prev) = row[x - 1] {
          result.add_neighbors(prev, *id);
        }
      }
    }
  }
  result
}

pub fn part1(input: &PrintShop) -> usize {
  input.find_moveable().len()
}

pub fn part2(input: &PrintShop) -> usize {
  let mut pending = input.find_moveable();
  let mut moved = vec![false; input.rolls.len()];
  let mut result = 0;
  while let Some(id) = pending.pop() {
    if !moved[id] &&
        input.rolls[id].neighbors.iter()
            .filter(|&id| !moved[*id]).count() < PrintShop::MIN_NEIGHBORS {
      moved[id] = true;
      result += 1;
      for neighbor in &input.rolls[id].neighbors {
        if !moved[*neighbor] {
          pending.push(*neighbor);
        }
      }
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
