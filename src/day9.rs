use std::vec::Vec;
use array2d::Array2D;
use itertools::Itertools;

type Position = usize;

#[derive(Debug)]
pub struct Tile {
  position: [Position; 2],
}

impl Tile {
  fn area(&self, other: &Tile) -> Position {
    (0..2).map(|i| self.position[i].abs_diff(other.position[i]) + 1).product()
  }
}

fn parse_int(s: &str) -> Result<Position, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

fn parse_line(line: &str) -> Result<Tile, String> {
  Ok(Tile {position: line.split(',').map(parse_int)
      .collect::<Result<Vec<_>, _>>()?.try_into()
      .map_err(|_| format!("wrong number of numbers - '{line}'"))?})
}

pub fn generator(input: &str) -> Vec<Tile> {
  input.lines().map(parse_line).collect::<Result<Vec<_>, _>>().expect("Bad input")
}

pub fn part1(input: &[Tile]) -> usize {
  (0..input.len()).combinations(2)
      .map(|pos| input[pos[0]].area(&input[pos[1]]))
      .max().expect("No pairs found!")
}

fn find_splits(input: &[Tile], dim: usize) -> Vec<Position> {
  let mut values = input.iter().map(|rt| rt.position[dim]).collect::<Vec<_>>();
  values.sort_unstable();
  values.dedup();
  values
}

#[derive(Clone,Debug,Eq,PartialEq)]
enum TileColor {
  Red (bool), // Does the wall go down from here?
  Vertical,
  Horizontal,
  Inside,
  None,
}

impl TileColor {
  fn flips_inside(&self) -> bool {
    match self {
      TileColor::Red(true) | TileColor::Vertical => true,
      _ => false,
    }
  }

  fn is_tiled(&self) -> bool {
    *self != TileColor::None
  }
}

struct SimplifiedGrid {
  grid: Array2D<TileColor>,
  mapped: Vec<Tile>,
}

impl SimplifiedGrid {
  fn new(input: &[Tile]) -> SimplifiedGrid {
    let x_splits = find_splits(input, 0);
    let y_splits = find_splits(input, 1);
    let grid =
        Array2D::filled_with(TileColor::None, y_splits.len(), x_splits.len());
    let mapped = input.iter().map(|t|
      Tile{position: [x_splits.binary_search(&t.position[0]).unwrap(),
        y_splits.binary_search(&t.position[1]).unwrap()]})
      .collect();
    SimplifiedGrid{grid, mapped}
  }

  fn draw_line(&mut self, left: usize, right: usize) {
    let (left, right) = (&self.mapped[left], &self.mapped[right]);
    if left.position[0] == right.position[0] {
      let x = left.position[0];
      let top = left.position[1].min(right.position[1]);
      let bottom = left.position[1].max(right.position[1]);
      self.grid.set(top, x, TileColor::Red(true)).expect("Cant set top");
      self.grid.set(bottom, x, TileColor::Red(false)).expect("Cant set bottom");
      for y in top+1..bottom {
        self.grid.set(y, x, TileColor::Vertical).expect("Cant set vertical");
      }
    } else {
      let y = left.position[1];
      let west = left.position[0].min(right.position[0]);
      let east = left.position[0].max(right.position[0]);
      for x in west+1..east {
        self.grid.set(y, x, TileColor::Horizontal).expect("Cant set horizontal");
      }
    }
  }

  fn fill_in(&mut self) {
    for y in 0..self.grid.num_rows() {
      let mut inside = false;
      for x in 0..self.grid.num_columns() {
        let spot = self.grid.get_mut(y, x).unwrap();
        if spot.flips_inside() {
          inside = !inside;
        } else if inside && *spot == TileColor::None {
          *spot = TileColor::Inside
        }
      }
    }
  }

  fn is_tiled(&self, first: usize, second: usize) -> bool {
    let (first, second) = (&self.mapped[first], &self.mapped[second]);
    let left = first.position[0].min(second.position[0]);
    let right = first.position[0].max(second.position[0]);
    let top = first.position[1].min(second.position[1]);
    let bottom = first.position[1].max(second.position[1]);
    for y in top..=bottom {
      for x in left..=right {
        if !self.grid.get(y, x).unwrap().is_tiled() {
          return false;
        }
      }
    }
    true
  }

  #[allow(unused)]
  fn print(&self) {
    for row in self.grid.rows_iter() {
      for spot in row {
        print!("{}", match spot {
          TileColor::Red(true) => "V",
          TileColor::Red(false) => "^",
          TileColor::Vertical => "|",
          TileColor::Horizontal => "-",
          TileColor::Inside => "X",
          TileColor::None => ".",
        });
      }
      println!();
    }
  }
}

pub fn part2(input: &[Tile]) -> usize {
  let mut grid = SimplifiedGrid::new(input);
  for (left, right) in (0..input.len()).tuple_windows() {
    grid.draw_line(left, right);
  }
  grid.draw_line(0, input.len() - 1);
  grid.fill_in();
  (0..input.len()).combinations(2)
      .filter(|pos| grid.is_tiled(pos[0], pos[1]))
      .map(|pos| input[pos[0]].area(&input[pos[1]]))
      .max().expect("No pairs found!")
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(50, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(24, part2(&data));
  }
}
