use std::vec::Vec;
use itertools::Itertools;
use itertools::EitherOrBoth::{Right, Both};

type Column = usize;

#[derive(Debug)]
pub struct Manifold {
  start: Column,
  splits: Vec<Vec<Column>>,
}

#[derive(Debug)]
enum Spot {
  Start(Column),
  Splitter(Column),
}

fn parse_line(s: &str) -> Vec<Spot> {
  s.chars().enumerate().flat_map(|(col, ch) |
      match ch {
        '.' => None,
        '^' => Some(Spot::Splitter(col)),
        'S' => Some(Spot::Start(col)),
        _ => panic!("Unknown spot: '{s}'")})
      .collect()
}

pub fn generator(input: &str) -> Manifold {
  let grid: Vec<Vec<Spot>> = input.lines().map(parse_line)
      .filter(|v| !v.is_empty())
      .collect();
  let start = if let [Spot::Start(col)] = grid[0][..] {
    col
  } else {
    panic!("No start column! in {:?}", grid[0])
  };
  let splits = grid[1..].iter()
      .map(|row| row.iter()
          .map(|s| if let Spot::Splitter(col) = s { col}
                          else {panic!("Unknown spot {s:?}")})
          .cloned()
          .collect())
      .collect();
  Manifold { start, splits }
}

pub fn part1(input: &Manifold) -> usize {
  let mut streams = vec![input.start];
  let mut result = 0;
  for row in &input.splits {
    let new_streams: Vec<Column> = row.iter().merge_join_by(streams.iter(), Ord::cmp)
        .flat_map(|cmp|
            match cmp {
              Both(col, _) => {result += 1; vec![*col - 1, *col + 1].into_iter()},
              Right(col) => vec![*col].into_iter(),
              _ => vec![].into_iter(),
            }).dedup().collect();
    streams = new_streams;
  }
  result
}

#[derive(Clone,Debug)]
struct ColumnCount {
  column: Column,
  count: usize,
}

impl ColumnCount {
  fn new(column: Column, count: usize) -> Self {
    ColumnCount { column, count }
  }
}

pub fn part2(input: &Manifold) -> usize {
  let mut streams = vec![ColumnCount::new(input.start, 1)];
  for row in &input.splits {
    let new_streams: Vec<ColumnCount> = row.iter()
        .merge_join_by(streams.iter(),
                |&left, &right| Column::cmp(left, &right.column))
        .flat_map(|cmp|
            match cmp {
              Both(_, right) =>
                vec![ColumnCount::new(right.column - 1, right.count),
                     ColumnCount::new(right.column + 1, right.count)].into_iter(),
              Right(col) => vec![(*col).clone()].into_iter(),
              _ => vec![].into_iter(),
            }).collect();
    streams = new_streams.into_iter().chunk_by(|stream| stream.column).into_iter()
        .map(|(col, itr)| ColumnCount::new(col, itr.into_iter()
            .map(|cc| cc.count).sum())).collect();
  }
  streams.iter().map(|cc| cc.count).sum()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(21, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(40, part2(&data));
  }
}
