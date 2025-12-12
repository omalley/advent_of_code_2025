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
  names: Vec<NamePair<'a>>,
  nodes: Vec<Node>,
  toposort: Vec<usize>,
}

impl<'a> Reactor<'a> {
  fn find_name(&self, name: &str) -> usize {
    self.names[self.names.binary_search_by_key(&name, |n| n.name).unwrap()].id
  }
}

const OUT_NAME: &str = "out";

fn toposort_visit(result: &mut Vec<usize>, node: usize, nodes: &[Node],
                  visited: &mut [bool]) {
  for child in &nodes[node].children {
    if *child < visited.len() && !visited[*child] {
      toposort_visit(result, *child, nodes, visited);
    }
  }
  visited[node] = true;
  result.push(node);
}

fn find_root_nodes(nodes: &[Node]) -> Vec<usize> {
  let mut is_root = vec![true; nodes.len()];
  for n in nodes.iter().flat_map(|n| n.children.iter()) {
    if *n < is_root.len() {
      is_root[*n] = false;
    }
  }
  is_root.iter().enumerate()
      .filter_map(|(i,e)| if *e { Some(i) } else { None })
      .collect()
}

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
  // Generate a toposort.
  let mut visited = vec![false; nodes.len()];
  let mut toposort = Vec::new();
  for root in find_root_nodes(&nodes) {
    toposort_visit(&mut toposort, root, &nodes, &mut visited);
  }
  toposort.reverse();
  Reactor{names, nodes, toposort}
}

const PART1_START_NAME: &str = "you";

pub fn part1(input: &Reactor) -> usize {
  let start = input.find_name(PART1_START_NAME);
  let mut streams: Vec<usize> = vec![0; input.names.len()];
  streams[start] = 1;
  for n in &input.toposort {
    if *n < streams.len() && streams[*n] > 0 {
      let new_streams = streams[*n];
      streams[*n] = 0;
      for child in &input.nodes[*n].children {
        streams[*child] += new_streams;
      }
    }
  }
  *streams.last().unwrap()
}

#[derive(Clone,Debug,Default)]
struct StreamCounts {
  orig: usize,
  dac: usize,
  fft: usize,
  both: usize,
}

impl StreamCounts {
  fn is_empty(&self) -> bool {
    self.orig == 0 && self.dac == 0 && self.fft == 0 && self.both == 0
  }

  fn add(&mut self, other: &StreamCounts) {
    self.orig += other.orig;
    self.dac += other.dac;
    self.fft += other.fft;
    self.both += other.both;
  }
}

const PART2_START_NAME: &str = "svr";

pub fn part2(input: &Reactor) -> usize {
  let start = input.find_name(PART2_START_NAME);
  let dac = input.find_name("dac");
  let fft = input.find_name("fft");
  let mut streams: Vec<StreamCounts> = vec![StreamCounts::default(); input.names.len()];
  streams[start] = StreamCounts{orig: 1, ..Default::default()};
  for n in &input.toposort {
    if *n < streams.len() && !streams[*n].is_empty() {
      let mut new_streams = streams[*n].clone();
      if *n == dac {
        new_streams.dac += new_streams.orig;
        new_streams.both += new_streams.fft;
        new_streams.orig = 0;
        new_streams.fft = 0;
      } else if *n == fft {
        new_streams.fft += new_streams.orig;
        new_streams.both += new_streams.dac;
        new_streams.orig = 0;
        new_streams.dac = 0;
      }
      streams[*n] = StreamCounts::default();
      for child in &input.nodes[*n].children {
        streams[*child].add(&new_streams);
      }
    }
  }
  streams.last().unwrap().both

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

  const INPUT2: &str =
"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

  #[test]
  fn test_part2() {
    let data = generator(INPUT2);
    assert_eq!(2, part2(&data));
  }
}
