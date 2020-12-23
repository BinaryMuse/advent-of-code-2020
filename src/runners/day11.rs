use super::coords::{Coordinate, Direction};
use super::grid::Grid;
use std::str::FromStr;
use strum::IntoEnumIterator;

type Coord = (isize, isize);

pub fn run(input: String, _args: &[String]) {
  let mut layout: Layout = input.parse().unwrap();
  while layout.tick(Rules::Part1) != 0 {}
  let count = layout.count_seated();
  println!("{} people are seated.", count);

  let mut layout: Layout = input.parse().unwrap();
  while layout.tick(Rules::Part2) != 0 {}
  let count = layout.count_seated();
  println!("With the modified rules, {} people are seated.", count);
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum SeatType {
  FLOOR,
  EMPTY,
  OCCUPIED,
}

impl FromStr for SeatType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "." => Ok(SeatType::FLOOR),
      "L" => Ok(SeatType::EMPTY),
      "#" => Ok(SeatType::OCCUPIED),
      _ => Err("Could not parse seat type".to_string()),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Rules {
  Part1,
  Part2,
}

fn part_1_rules(layout: &Layout, coord: Coord, current: SeatType) -> Option<SeatType> {
  if current == SeatType::FLOOR {
    return None;
  }

  let occupied_adjacent = coord
    .neighbors()
    .iter()
    .map(|c| layout.items.get(c))
    .filter(|&v| matches!(v, Some(&s) if s == SeatType::OCCUPIED))
    .count();

  match (current, occupied_adjacent) {
    (SeatType::EMPTY, 0) => Some(SeatType::OCCUPIED),
    (SeatType::OCCUPIED, n) if n >= 4 => Some(SeatType::EMPTY),
    _ => None,
  }
}

fn part_2_rules(layout: &Layout, coord: Coord, current: SeatType) -> Option<SeatType> {
  if current == SeatType::FLOOR {
    return None;
  }

  let visible_taken = layout
    .seen_from(coord)
    .iter()
    .filter(|&coord| matches!(layout.items.get(coord), Some(SeatType::OCCUPIED)))
    .count();

  match (current, visible_taken) {
    (SeatType::EMPTY, 0) => Some(SeatType::OCCUPIED),
    (SeatType::OCCUPIED, n) if n >= 5 => Some(SeatType::EMPTY),
    _ => None,
  }
}

#[derive(Debug, Clone)]
struct Layout {
  items: Grid<SeatType>,
}

impl Layout {
  fn new() -> Self {
    Self { items: Grid::new() }
  }

  fn tick(&mut self, rules: Rules) -> usize {
    let mut changes: Vec<(Coord, SeatType)> = vec![];

    for (&key, &value) in self.items.iter() {
      let change = match rules {
        Rules::Part1 => part_1_rules(&self, key, value),
        Rules::Part2 => part_2_rules(&self, key, value),
      };

      if let Some(new_value) = change {
        changes.push((key, new_value));
      }
    }

    for &(coord, new_state) in changes.iter() {
      self.items.insert(coord, new_state);
    }

    changes.len()
  }

  fn seen_from(&self, coord: Coord) -> Vec<Coord> {
    Direction::iter()
      .map(|dir| {
        coord
          .toward(dir)
          // Coordinates outside of our grid's working space will return `None`
          .take_while(|coord| self.items.get(coord).is_some())
          .find(|coord| {
            let item = self.items.get(coord);
            matches!(item, Some(SeatType::OCCUPIED) | Some(SeatType::EMPTY))
          })
      })
      .filter(Option::is_some)
      .map(Option::unwrap)
      .collect()
  }

  fn count_seated(&self) -> usize {
    self
      .items
      .iter()
      .filter(|(_, &v)| v == SeatType::OCCUPIED)
      .count()
  }
}

impl FromStr for Layout {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut layout = Layout::new();

    for (row, line) in s.lines().enumerate() {
      for (col, chr) in line.chars().enumerate() {
        let seat: SeatType = chr.to_string().parse()?;
        let x = col as isize;
        let y = row as isize;
        layout.items.insert((x, y), seat);
      }
    }

    Ok(layout)
  }
}

#[test]
fn test_seat_rules() {
  let input = super::common::get_input("11_sample").unwrap();
  let mut layout: Layout = input.parse().unwrap();
  assert_eq!(layout.items.get(&(0, 0)), Some(&SeatType::EMPTY));
  assert_eq!(layout.items.get(&(1, 0)), Some(&SeatType::FLOOR));
  assert_eq!(layout.items.get(&(0, 1)), Some(&SeatType::EMPTY));
  assert_eq!(layout.items.get(&(9, 2)), Some(&SeatType::FLOOR));
  assert_eq!(layout.items.get(&(10, 2)), None);

  layout.tick(Rules::Part1);

  assert_eq!(layout.items.get(&(0, 0)), Some(&SeatType::OCCUPIED));
  assert_eq!(layout.items.get(&(1, 0)), Some(&SeatType::FLOOR));
  assert_eq!(layout.items.get(&(0, 1)), Some(&SeatType::OCCUPIED));
  assert_eq!(layout.items.get(&(9, 2)), Some(&SeatType::FLOOR));
  assert_eq!(layout.items.get(&(10, 2)), None);

  while layout.tick(Rules::Part1) != 0 {}
  assert_eq!(layout.count_seated(), 37);

  // Part 2
  let input = super::common::get_input("11_sample").unwrap();
  let mut layout: Layout = input.parse().unwrap();
  layout.tick(Rules::Part2);
  assert_eq!(layout.items.get(&(0, 0)), Some(&SeatType::OCCUPIED));
  assert_eq!(layout.items.get(&(1, 0)), Some(&SeatType::FLOOR));
  assert_eq!(layout.items.get(&(0, 1)), Some(&SeatType::OCCUPIED));
  assert_eq!(layout.items.get(&(9, 2)), Some(&SeatType::FLOOR));
  assert_eq!(layout.items.get(&(10, 2)), None);

  while layout.tick(Rules::Part2) != 0 {}
  assert_eq!(layout.count_seated(), 26);
}
