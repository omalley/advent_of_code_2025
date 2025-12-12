use std::vec::Vec;
use itertools::Itertools;
use smallvec::SmallVec;

type LightMask = SmallVec<[bool; 16]>;
type Button = SmallVec<[usize; 8]>;

#[derive(Debug)]
pub struct Machine {
  goal: LightMask,
  buttons: Vec<Button>,
  energy: Vec<usize>,
}

impl Machine {
  fn parse_button(phrase: &str) -> Result<Button, String> {
    let phrase = phrase.strip_prefix("(")
        .ok_or(format!("Missing paren: '{phrase}'"))?;
    let phrase = phrase.strip_suffix(")")
        .ok_or(format!("Missing paren: '{phrase}'"))?;
    phrase.split(',').map(parse_int).collect::<Result<Button, _>>()
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
        .map(Self::parse_button)
        .collect::<Result<Vec<_>, _>>()?;
    buttons.sort_unstable();
    let energy = Self::parse_enery(energy_str)?;
    Ok(Machine{goal, buttons, energy})
  }

  fn lights_to_mask(light_mask: &LightMask) -> u64 {
    light_mask.iter().fold(0, |acc, &b| acc * 2 + if b {1} else {0})
  }

  fn button_to_mask(button: &Button, bits: usize) -> u64 {
    button.iter().map(|b| 2_u64.pow((bits - *b - 1) as u32)).sum()
  }
}

fn parse_int(s: &str) -> Result<usize, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

pub fn generator(input: &str) -> Vec<Machine> {
  input.lines().map(Machine::parse).collect::<Result<Vec<_>, _>>().expect("Bad input")
}

fn solve_part1(machine: &Machine) -> usize {
  let goal = Machine::lights_to_mask(&machine.goal);
  let buttons : Vec<u64> = machine.buttons.iter()
      .map(|b| Machine::button_to_mask(b, machine.goal.len()))
      .collect();
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
  input.iter().map(solve_part1).sum()
}

enum BestButton {
  Only(usize, usize), // must pick given column with given number
  Best(usize), // should pick given column
  None, // nothing will work
}

fn best_button(goal: &[usize], buttons: &[Button], remaining: &[bool]) -> BestButton {
  // how many remaining buttons can hit each lamp?
  // the second component is the first such button
  let mut buttons_by_lamp: Vec<(usize, usize)> = vec![(0, 0); goal.len()];
  for b in 0..buttons.len() {
    if remaining[b] {
      for lamp in &buttons[b] {
        if buttons_by_lamp[*lamp].0 == 0 {
          buttons_by_lamp[*lamp].1 = b;
        }
        buttons_by_lamp[*lamp].0 += 1;
      }
    }
  }
  let mut best_lamp: Option<(usize,usize)> = None;
  for (lamp, (count, button)) in buttons_by_lamp.iter().enumerate() {
    if *count == 0 && goal[lamp] != 0 {
      return BestButton::None;
    } else if *count == 1 {
      return BestButton::Only(*button, goal[lamp]);
    } else if let Some((prev_lamp, prev_count)) = best_lamp &&
        (prev_count < *count || (prev_count == *count && goal[prev_lamp] > goal[lamp])) {
      // pass
    } else if *count != 0 {
      best_lamp = Some((lamp, *count));
    }
  }
  if let Some((lamp, _)) = best_lamp {
    BestButton::Best(buttons_by_lamp[lamp].1)
  } else {
    BestButton::None
  }
}

fn solve_part2_by_button(goal: &[usize], buttons: &[Button], pushes: usize,
                         remaining: &mut [bool]) -> bool {
  if pushes == 0 || remaining.iter().all(|b| !*b) {
    goal.iter().all(|x| *x == 0)
  } else if pushes < *goal.iter().max().unwrap() {
    false
  } else {
    let (best_button, min_pushes) = match best_button(goal, buttons, remaining) {
      BestButton::Only(button, count) => (button, count),
      BestButton::Best(button) => (button, 0),
      BestButton::None => return false,
    };
    assert!(remaining[best_button]);
    remaining[best_button] = false;
    let max_pushes = pushes.min(buttons[best_button].iter()
        .map(|i| goal[*i]).min().unwrap());
    for our_pushes in (min_pushes..=max_pushes).rev() {
      let mut sub_goal = goal.to_owned();
      for b in &buttons[best_button] {
        sub_goal[*b] -= our_pushes;
      }
      if solve_part2_by_button(&sub_goal, buttons, pushes - our_pushes, remaining) {
        return true;
      }
    }
    remaining[best_button] = true;
    false
  }
}

fn solve_part2(machine: &Machine) -> usize {
  println!("Starting {machine:?}");
  let mut remaining = vec![true; machine.buttons.len()];
  // we need at least the minimum of the energy and no worse than the sum.
  for pushes in *(machine.energy.iter().max().unwrap())..=
      machine.energy.iter().sum::<usize>() {
    println!("Checking {pushes} pushes");
    if solve_part2_by_button(&machine.energy, &machine.buttons, pushes, &mut remaining) {
      return pushes;
    }
  }
  panic!("No solution found");
}

pub fn part2(input: &[Machine]) -> usize {
  input.iter().map(solve_part2).sum()
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
    assert_eq!(33, part2(&data));
  }
}
