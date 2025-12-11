#[derive(Debug,Default)]
pub struct Node {
  children: Vec<usize>,
}

#[derive(Debug,Eq,Ord,PartialEq,PartialOrd)]
pub struct NamePair<'a> {
  name: &'a str,
  id: usize,
}

#[derive(Debug)]
pub struct Reactor<'a> {
  start: usize,
  names: Vec<NamePair<'a>>,
  nodes: Vec<Node>,
}

const START_NAME: &str = "you";
const OUT_NAME: &str = "out";

pub fn generator(input: &str) -> Reactor<'_> {
  let mut nodes = Vec::new();
  let mut names = Vec::new();
  for line in input.lines() {
    let (name, _) = line.split_once(':')
        .expect("Can't find name in {line}");
    names.push(NamePair{name, id: names.len()});
    nodes.push(Node::default());
  }
  names.push(NamePair{name: OUT_NAME, id: names.len()});
  names.sort_unstable();
  for (id, line) in input.lines().enumerate() {
    let (_, child_str) = line.split_once(':').unwrap();
    nodes[id].children.extend(child_str.split_whitespace()
        .map(|c| names[names.binary_search_by_key(&c, |n| n.name).unwrap()].id));
  }
  let start = names[names.binary_search_by_key(&START_NAME,
                                         |n| n.name).unwrap()].id;
  Reactor{start, names, nodes}
}

pub fn part1(input: &Reactor) -> usize {
  let mut streams: Vec<usize> = vec![0; input.names.len()];
  streams[input.start] = 1;
  while let Some(i) = streams.iter().position(|s| *s > 0) && i < input.nodes.len() {
    let new_streams = streams[i];
    streams[i] = 0;
    for child in &input.nodes[i].children {
      streams[*child] += new_streams;
    }
  }
  *streams.last().unwrap()
}

pub fn part2(input: &Reactor) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(5, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(6, part2(&data));
  }
}
