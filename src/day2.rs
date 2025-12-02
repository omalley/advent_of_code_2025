use core::array;
use itertools::Itertools;
use lazy_static::lazy_static;
use num_integer::Integer;
use std::ops::RangeInclusive;

type Product = u64;

lazy_static! {
  static ref POWER_10: [Product; 18] = array::from_fn(|i| (10 as Product).pow(i as u32));
}

fn parse_int(s: &str) -> Result<Product, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

fn parse_line(s: &str) -> Result<RangeInclusive<Product>, String> {
  let (left, right) = s.trim().split_once('-')
      .ok_or("Can't parse range '{s}'")?;
  Ok(RangeInclusive::new(parse_int(left)?.max(11), parse_int(right)?))
}

pub fn generator(input: &str) -> Vec<RangeInclusive<Product>> {
  input.split(',').map(parse_line).try_collect().expect("Can't parse input")
}

fn is_invalid(num: Product) -> bool {
  let digits = num.ilog10() + 1;
  if digits.is_even() {
    let split = POWER_10[digits as usize/ 2];
    return (num % split) == (num / split)
  }
  false
}

fn sum_matching<F>(input: &[RangeInclusive<Product>], filter: F) -> Product
    where F: Fn(Product) -> bool {
  input.iter().flat_map(|r| r.clone()).filter(|r| filter(*r)).sum()
}

pub fn part1(input: &[RangeInclusive<Product>]) -> Product {
  sum_matching(input, is_invalid)
}

/// Does the given number repeat digits given the power of 10 in split?
fn digits_repeat(num: Product, split: Product) -> bool {
  let goal = num % split;
  let mut remainder = num / split;
  while remainder != 0 {
    if remainder % split != goal {
      return false;
    }
    remainder /= split;
  }
  true
}

fn is_invalid2(num: Product) -> bool {
  let digits = num.ilog10() + 1;
  for part_digit in (1..=(digits/2)).rev() {
    if digits.is_multiple_of(part_digit) &&
        digits_repeat(num, POWER_10[part_digit as usize]) {
      return true;
    }
  }
  false
}

pub fn part2(input: &[RangeInclusive<Product>]) -> Product {
  sum_matching(input, is_invalid2)
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
