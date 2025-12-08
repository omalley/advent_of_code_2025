use std::vec::Vec;
use itertools::Itertools;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

type Position = usize;

#[derive(Debug)]
pub struct Junction {
  position: [Position; 3],
}

impl Junction {
  fn distance_sqr(&self, other: &Junction) -> Position {
    (0..3).map(|i| (self.position[i].abs_diff(other.position[i])).pow(2)).sum()
  }
}

fn parse_int(s: &str) -> Result<Position, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

fn parse_line(line: &str) -> Result<Junction, String> {
  Ok(Junction{position: line.split(',').map(parse_int)
      .collect::<Result<Vec<_>, _>>()?.try_into()
      .map_err(|_| format!("wrong number of numbers - '{line}'"))?})
}

pub fn generator(input: &str) -> Vec<Junction> {
  input.lines().map(parse_line).collect::<Result<Vec<_>, _>>().expect("Bad input")
}

fn calculate_best(input: &[Junction], limit: usize) -> usize {
  let mut distances: Vec<(Position,usize,usize)> = (0..input.len()).combinations(2)
      .map(|pos| (input[pos[0]].distance_sqr(&input[pos[1]]), pos[0], pos[1] ))
      .collect();
  distances.sort_unstable_by(|x, y| (x.0).cmp(&y.0));
  let mut networks = QuickUnionUf::<UnionBySize>::new(input.len());
  for (_, left, right) in distances.iter().take(limit) {
    networks.union(*left, *right);
  }
  // Find the set leaders
  let leaders = (0..input.len())
      .filter(|i| networks
          .find(*i) == *i).collect::<Vec<_>>();
  let mut sizes:Vec<usize> = leaders.iter()
      .map(|i| networks.get(*i).size())
      .collect();
  sizes.sort_unstable_by(|a, b| b.cmp(a));
  sizes.iter().take(3).product()
}

pub fn part1(input: &[Junction]) -> usize {
  calculate_best(input, 1000)
}

pub fn part2(input: &[Junction]) -> usize {
  let mut distances: Vec<(Position,usize,usize)> = (0..input.len()).combinations(2)
      .map(|pos| (input[pos[0]].distance_sqr(&input[pos[1]]), pos[0], pos[1] ))
      .collect();
  distances.sort_unstable_by(|x, y| (x.0).cmp(&y.0));
  let mut networks = QuickUnionUf::<UnionBySize>::new(input.len());
  let mut remaining = input.len() - 1;
  for (_, left, right) in distances.iter() {
    if networks.union(*left, *right) {
      remaining -= 1;
      if remaining == 0 {
        return input[*left].position[0] * input[*right].position[0];
      }
    }
  }
  panic!("Didn't find a solution");
}

#[cfg(test)]
mod tests {
  use super::{generator, calculate_best, part2};

  const INPUT: &str =
"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(40, calculate_best(&data, 10));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(25272, part2(&data));
  }
}
