use std::collections::VecDeque;
use std::vec::Vec;

type LightMask = u16;

#[derive(Debug)]
pub struct Machine {
  mask: LightMask,
  goal: LightMask,
  buttons: Vec<LightMask>,
  energy: Vec<u64>,
}

impl Machine {
  fn parse_button(phrase: &str, bits: usize) -> Result<LightMask, String> {
    let phrase = phrase.strip_prefix("(")
        .ok_or(format!("Missing paren: '{phrase}'"))?;
    let phrase = phrase.strip_suffix(")")
        .ok_or(format!("Missing paren: '{phrase}'"))?;
    let nums = phrase.split(',').map(parse_int).collect::<Result<Vec<_>, _>>()?;
    Ok(nums.iter().fold(0_u16,
                        |acc, num| acc | 2_u16.pow(bits as u32 - *num as u32 - 1)))
  }

  fn parse_enery(phrase: &str) -> Result<Vec<u64>, String> {
    let phrase = phrase.strip_suffix("}")
        .ok_or(format!("Missing brace: '{phrase}'"))?;
    phrase.split(",").map(parse_int).collect::<Result<Vec<_>, _>>()
  }

  fn parse(line: &str) -> Result<Machine,String> {
    let (goal_str, rest) = line.split_once("] ")
        .ok_or(format!("Invalid goal: {line}"))?;
    let (button_str, energy_str) = rest.split_once(" {")
        .ok_or(format!("Invalid button: {rest}"))?;
    let mask = 2_u16.pow((goal_str.len() - 1) as u32) as u16 - 1;
    let goal = goal_str[1..].chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .fold(0, |acc, d| acc * 2 + d);
    let mut buttons = button_str.split(" ")
        .map(|s| Self::parse_button(s, goal_str.len() - 1))
        .collect::<Result<Vec<_>, _>>()?;
    buttons.sort_unstable();
    let energy = Self::parse_enery(energy_str)?;
    Ok(Machine{mask, goal, buttons, energy})
  }
}

fn parse_int(s: &str) -> Result<u64, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

pub fn generator(input: &str) -> Vec<Machine> {
  input.lines().map(Machine::parse).collect::<Result<Vec<_>, _>>().expect("Bad input")
}

fn find_part1(machine: &Machine) -> usize {
  let mut pending: VecDeque<(usize, LightMask)> = VecDeque::new();
  pending.push_back((0, 0));
  while let Some((count, lights)) = pending.pop_front() {
    if lights == machine.goal {
      return count;
    }
    for button in &machine.buttons {
      pending.push_back((count + 1, button ^ lights));
    }
  }
  unreachable!()
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
