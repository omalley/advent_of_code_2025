use std::collections::VecDeque;
use std::vec::Vec;
use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

type LightMask = SmallVec<[bool; 16]>;

#[derive(Debug)]
pub struct Machine {
  goal: LightMask,
  buttons: Vec<LightMask>,
  energy: Vec<usize>,
}

impl Machine {
  fn parse_button(phrase: &str, bits: usize) -> Result<LightMask, String> {
    let phrase = phrase.strip_prefix("(")
        .ok_or(format!("Missing paren: '{phrase}'"))?;
    let phrase = phrase.strip_suffix(")")
        .ok_or(format!("Missing paren: '{phrase}'"))?;
    let mut result = smallvec![false; bits];
    for num in phrase.split(',').map(parse_int).collect::<Result<Vec<_>, _>>()? {
      result[num] = true;
    }
    Ok(result)
  }

  fn parse_enery(phrase: &str) -> Result<Vec<usize>, String> {
    let phrase = phrase.strip_suffix("}")
        .ok_or(format!("Missing brace: '{phrase}'"))?;
    phrase.split(",").map(parse_int).collect::<Result<Vec<_>, _>>()
  }

  fn parse(line: &str) -> Result<Machine,String> {
    let (goal_str, rest) = line.split_once("] ")
        .ok_or(format!("Invalid goal: {line}"))?;
    let (button_str, energy_str) = rest.split_once(" {")
        .ok_or(format!("Invalid button: {rest}"))?;
    let goal: LightMask = goal_str[1..].chars()
        .map(|c| c == '#')
        .collect();
    let mut buttons = button_str.split(" ")
        .map(|s| Self::parse_button(s, goal_str.len() - 1))
        .collect::<Result<Vec<_>, _>>()?;
    buttons.sort_unstable();
    let energy = Self::parse_enery(energy_str)?;
    Ok(Machine{goal, buttons, energy})
  }

  fn to_mask(light_mask: &LightMask) -> u64 {
    light_mask.iter().fold(0, |acc, &b| acc * 2 + if b {1} else {0})
  }
}

fn parse_int(s: &str) -> Result<usize, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

pub fn generator(input: &str) -> Vec<Machine> {
  input.lines().map(Machine::parse).collect::<Result<Vec<_>, _>>().expect("Bad input")
}

fn find_part1(machine: &Machine) -> usize {
  let goal = Machine::to_mask(&machine.goal);
  let buttons : Vec<u64> = machine.buttons.iter().map(|b| Machine::to_mask(b)).collect();
  for pushes in 0..buttons.len() {
    for combo in buttons.iter().combinations(pushes) {
      if goal == combo.iter().fold(0, |acc, &b| acc ^ b) {
        return pushes;
      }
    }
  }
  panic!("No solution found");
}

pub fn part1(input: &[Machine]) -> usize {
  input.iter().map(find_part1).sum()
}


pub fn part2(input: &[Machine]) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(7, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(0, part2(&data));
  }
}
