use std::ops::RangeInclusive;
use std::vec::Vec;

type IngredientId = usize;

#[derive(Debug)]
pub struct Kitchen {
  good: Vec<RangeInclusive<IngredientId>>,
  ingredients: Vec<IngredientId>,
}

impl Kitchen {
  fn dedup_ranges(good: &mut Vec<RangeInclusive<IngredientId>>) {
    good.sort_unstable_by_key(|rng| *rng.start());
    let mut deduped: Vec<RangeInclusive<IngredientId>> = Vec::with_capacity(good.len());
    let mut current: Option<RangeInclusive<IngredientId>> = None;
    for orig in good.iter() {
      if let Some(prev) = &current {
        if prev.contains(orig.start()) {
          // merge the two ranges
          current = Some(*(prev.start())..=(*prev.end()).max(*orig.end()));
        } else {
          deduped.push(prev.clone());
          current = Some(orig.clone());
        }
      } else {
        current = Some(orig.clone());
      }
    }
    if let Some(prev) = current {
      deduped.push(prev);
    }
    good.clear();
    good.append(&mut deduped);
  }

  fn is_good(&self, ingredient: IngredientId) -> bool {
    match self.good.binary_search_by_key(&ingredient,
                                         |rng| *rng.start()) {
      Ok(_) => true,
      Err(following) => if following == 0 { false }
                               else { self.good[following - 1].contains(&ingredient) },
    }
  }
}

fn parse_int(s: &str) -> Result<IngredientId, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

fn parse_range(line: &str) -> Result<RangeInclusive<IngredientId>, String> {
  let (start, end) = line.split_once("-").ok_or("Can't parse range")?;
  Ok((parse_int(start)?)..=(parse_int(end)?))
}

pub fn generator(input: &str) -> Kitchen {
  let (good, ingredients) = input.split_once("\n\n")
      .expect("No ingredient list");
  let mut good: Vec<RangeInclusive<IngredientId>> =
      good.lines().map(parse_range).collect::<Result<_, _>>()
          .expect("Can't parse good list");
  Kitchen::dedup_ranges(&mut good);
  let ingredients = ingredients.lines().map(parse_int).collect::<Result<_, _>>()
      .expect("Can't parse ingredient list");
  Kitchen { good, ingredients }
}

pub fn part1(input: &Kitchen) -> usize {
  input.ingredients.iter().filter(|id| input.is_good(**id)).count()
}

pub fn part2(input: &Kitchen) -> usize {
  input.good.iter().map(|range| range.end() - range.start() + 1).sum()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(3, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(14, part2(&data));
  }
}
