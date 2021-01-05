use std::collections::HashMap;

pub fn run(input: String, _args: &[String]) {
  {
    let mut game = MemoryGame::new(parse_input(&input));
    let result = game.nth(2020 - 1).unwrap();
    println!("The 2020th value is {}", result);
  }

  {
    // Runs in reasonable time in release mode
    let mut game = MemoryGame::new(parse_input(&input));
    let result = game.nth(30000000 - 1).unwrap();
    println!("The 30000000th value is {}", result);
  }
}

fn parse_input(s: &str) -> Vec<u32> {
  s.split(',').map(|s| s.parse::<u32>().unwrap()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SpokenHistory {
  Never,
  Once(usize),
  Twice(usize, usize),
}

struct MemoryGame {
  turn: usize,
  start: Vec<u32>,
  last: u32,
  history: Vec<SpokenHistory>,
}

impl MemoryGame {
  fn new(values: Vec<u32>) -> Self {
    Self {
      turn: 0,
      start: values,
      last: 0,
      history: Vec::with_capacity(2000),
    }
  }

  fn speak(&mut self, value: u32) -> Option<u32> {
    let idx = value as usize;
    self.last = value;
    let history = self.history.get(idx);
    let new_history = match history {
      None => SpokenHistory::Once(self.turn),
      Some(SpokenHistory::Never) => SpokenHistory::Once(self.turn),
      Some(SpokenHistory::Once(a)) => SpokenHistory::Twice(*a, self.turn),
      Some(SpokenHistory::Twice(_, b)) => SpokenHistory::Twice(*b, self.turn),
    };

    if idx >= self.history.len() {
      let diff = idx - self.history.len();
      self
        .history
        .resize(self.history.len() + diff * 2 + 1, SpokenHistory::Never)
    }
    self.history[value as usize] = new_history;
    self.turn += 1;

    Some(value)
  }
}

impl Iterator for MemoryGame {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.turn < self.start.len() {
      self.speak(self.start[self.turn])
    } else {
      let history = self.history.get(self.last as usize).unwrap();
      let to_speak = match history {
        SpokenHistory::Never => panic!("Expected a history"),
        SpokenHistory::Once(_) => 0,
        SpokenHistory::Twice(a, b) => b - a,
      };
      self.speak(to_speak as u32)
    }
  }
}

#[test]
fn test_speaking_game() {
  let mut game = MemoryGame::new(parse_input("0,3,6"));
  assert_eq!(game.next(), Some(0));
  assert_eq!(game.next(), Some(3));
  assert_eq!(game.next(), Some(6));
  assert_eq!(game.next(), Some(0));
  assert_eq!(game.next(), Some(3));
  assert_eq!(game.next(), Some(3));
  assert_eq!(game.next(), Some(1));
  assert_eq!(game.next(), Some(0));
  assert_eq!(game.next(), Some(4));
  assert_eq!(game.next(), Some(0));
}

#[test]
fn test_part_1() {
  let mut game = MemoryGame::new(parse_input("0,3,6"));
  assert_eq!(game.nth(2019), Some(436));

  let cases = vec![
    ("1,3,2", 1),
    ("2,1,3", 10),
    ("1,2,3", 27),
    ("2,3,1", 78),
    ("3,2,1", 438),
    ("3,1,2", 1836),
  ];

  for (input, expected) in cases {
    let mut game = MemoryGame::new(parse_input(input));
    assert_eq!(game.nth(2020 - 1), Some(expected));
  }
}

// These tests take a long time in debug mode
// #[test]
// fn test_part_2() {
//   let cases = vec![
//     ("0,3,6", 175594),
//     ("1,3,2", 2578),
//     ("2,1,3", 27),
//     ("1,2,3", 78),
//     ("2,3,1", 438),
//     ("3,2,1", 1836),
//     ("3,1,2", 1836),
//   ];

//   for (input, expected) in cases {
//     let mut game = MemoryGame::new(parse_input(input));
//     assert_eq!(game.nth(30000000 - 1), Some(expected));
//   }
// }
