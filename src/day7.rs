use std::vec::Vec;

type Column = usize;

#[derive(Debug)]
pub struct Manifold {
  start: Column,
  splits: Vec<Vec<bool>>,
}

const START: char = 'S';
const SPLIT: char = '^';

pub fn generator(input: &str) -> Manifold {
  let mut lines = input.lines();
  let start = lines.next().expect("Missing first line!")
      .chars().position(|c| c == START).expect("Can't find start position");
  let splits = lines.map(|line| line.chars()
        .map(|ch| ch == SPLIT)
        .collect())
      .filter(|row: &Vec<bool>| row.iter().any(|b| *b))
      .collect();
  Manifold{start, splits}
}

pub fn part1(input: &Manifold) -> usize {
  // keep track of where there are streams
  let mut streams = vec![false; input.splits[0].len() + 1];
  streams[input.start] = true;
  // how many splitters have we reached?
  let mut result = 0;
  for row in &input.splits {
    for (x, split) in row.iter().enumerate() {
      if *split && streams[x] {
        result += 1;
        streams[x-1] = true;
        streams[x] = false;
        streams[x+1] = true;
      }
    }
  }
  result
}

pub fn part2(input: &Manifold) -> usize {
  // keep track of where there are streams
  let mut streams = vec![0; input.splits[0].len() + 1];
  streams[input.start] = 1;
  for row in &input.splits {
    let mut prev = 0;
    for (x, split) in row.iter().enumerate() {
      if *split {
        let incoming = streams[x];
        streams[x-1] += incoming;
        streams[x] = prev;
        prev = incoming;
      } else {
        streams[x] += prev;
        prev = 0;
      }
    }
  }
  streams.iter().sum()
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
