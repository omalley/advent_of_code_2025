use core::array;
use lazy_static::lazy_static;
use num_integer::Integer;
use smallvec::SmallVec;
use std::ops::RangeInclusive;

type ProductId = u64;

lazy_static! {
  /// Generate a table of the powers of 10.
  static ref POWER_10: [ProductId; 18] =
    array::from_fn(|i| (10 as ProductId).pow(i as u32));
}

fn parse_int(s: &str) -> Result<ProductId, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

#[derive(Clone,Debug)]
pub struct RangeSlice {
  range: RangeInclusive<ProductId>,
  digits: usize,
}

type RangeSmallVec = SmallVec<[RangeSlice; 2]>;

/// Parse a given range in to our internal form.
/// We split the ranges in to sub-ranges that have consistent number of digits.
/// Since 0 is annoying for ilog10, I forced 1 as a lower bound.
fn parse_range(s: &str) -> Result<RangeSmallVec, String> {
  let (left, right) = s.trim().split_once('-')
      .ok_or("Can't parse range '{s}'")?;
  let start = parse_int(left)?.max(1);
  let end = parse_int(right)?.max(1);
  let mut result: RangeSmallVec = SmallVec::new();
  let start_digits = start.ilog10() as usize + 1;
  let end_digits = end.ilog10() as usize + 1;
  // Is the whole range a single number of digits?
  if start_digits == end_digits {
    result.push(RangeSlice{range: start..=end, digits: start_digits});
  } else {
    // Add the range for the start_digits.
    result.push(RangeSlice{range: start..=(POWER_10[start_digits] - 1),
      digits: start_digits});
    // Add the middle ranges
    for digits in (start_digits+1)..end_digits {
      result.push(RangeSlice{range: POWER_10[digits-1]..=(POWER_10[digits] - 1),
        digits});
    }
    // Add the range for the end_digits
    result.push(RangeSlice{range: POWER_10[end_digits-1]..=end, digits: end_digits});
  }
  Ok(result)
}

/// Convert the input into a list of RangeSlices.
pub fn generator(input: &str) -> Vec<RangeSlice> {
  input.split(',').map(parse_range)
      .collect::<Result<Vec<RangeSmallVec>,String>>()
      .expect("Can't parse input")
      .iter()
      .flat_map(|rsv| rsv.iter())
      .cloned()
      .collect()
}

pub fn part1(ranges: &[RangeSlice]) -> ProductId {
  let mut result = 0;
  for range in ranges {
    // Ignore any ranges that have odd number of digits in their numbers.
    if range.digits.is_even() {
      // Get the power of 10 that splits the numbers in half.
      let split = POWER_10[range.digits/2];
      // Check each possible prefix to see if the resulting number is in the range.
      for prefix in (range.range.start()/split)..=(range.range.end()/split) {
        if range.range.contains(&(prefix * (split + 1))) {
          // If the number is in the range, add it into the result.
          result += prefix * (split + 1);
        }
      }
    }
  }
  result
}

/// Given a split power of 10 and a number of repetitions, make the
/// matching expanded mask.
/// create_expanded_mask(100, 2) = 101
/// create_expanded_mask(1000, 3) = 1001001
fn create_expanded_mask(split: ProductId, repetitions: usize) -> ProductId {
  let mut result = split + 1;
  for _ in 2..repetitions {
    result = result * split + 1;
  }
  result
}

/// Does the given num of length digits have a repeating pattern?
/// We need to ignore prefixes with repeating patterns, because they will be counted
/// with the smaller pattern.
fn has_repeats(num: ProductId, digits: usize) -> bool {
  for part_digits in 1..=(digits/2) {
    if digits.is_multiple_of(part_digits) &&
        num == (num / POWER_10[digits - part_digits]) *
            create_expanded_mask(POWER_10[part_digits], digits / part_digits) {
      return true;
    }
  }
  false
}

pub fn part2(ranges: &[RangeSlice]) -> ProductId {
  let mut result = 0;
  for range in ranges {
    // For each possible number of pattern digits
    for pat_digits in 1..=(range.digits/2) {
      // It must be an exact multiple of the full number of digits.
      if range.digits.is_multiple_of(pat_digits) {
        let expanded_mask = create_expanded_mask(POWER_10[pat_digits],
                                                 range.digits / pat_digits);
        let prefix_mask = POWER_10[range.digits - pat_digits];
        for prefix in (range.range.start()/prefix_mask)..=(range.range.end()/prefix_mask) {
          if !has_repeats(prefix, pat_digits) &&
              range.range.contains(&(expanded_mask * prefix)) {
            result += expanded_mask * prefix;
          }
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
"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(1227775554, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(4174379265, part2(&data));
  }
}
