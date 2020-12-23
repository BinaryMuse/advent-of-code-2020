use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Direction {
  N,
  S,
  E,
  W,
  NW,
  NE,
  SW,
  SE,
}

impl Direction {
  pub fn delta(&self) -> (isize, isize) {
    match self {
      Direction::N => (0, 1),
      Direction::S => (0, -1),
      Direction::E => (1, 0),
      Direction::W => (-1, 0),
      Direction::NW => (-1, 1),
      Direction::NE => (1, 1),
      Direction::SW => (-1, -1),
      Direction::SE => (1, -1),
    }
  }

  pub fn right_90(&self) -> Self {
    match self {
      Direction::N => Direction::E,
      Direction::S => Direction::W,
      Direction::E => Direction::S,
      Direction::W => Direction::N,
      Direction::NW => Direction::NE,
      Direction::NE => Direction::SE,
      Direction::SW => Direction::NW,
      Direction::SE => Direction::SW,
    }
  }

  pub fn left_90(&self) -> Self {
    match self {
      Direction::N => Direction::W,
      Direction::S => Direction::E,
      Direction::E => Direction::N,
      Direction::W => Direction::S,
      Direction::NW => Direction::SW,
      Direction::NE => Direction::NW,
      Direction::SW => Direction::SE,
      Direction::SE => Direction::NE,
    }
  }
}

pub trait Coordinate {
  type Item;

  fn neighbors(&self) -> Vec<Self::Item>;
  fn toward(&self, direction: Direction) -> TowardIter;
}

impl Coordinate for (isize, isize) {
  type Item = (isize, isize);

  fn neighbors(&self) -> Vec<Self::Item> {
    let mut result: Vec<Self::Item> = Vec::with_capacity(8);

    for dir in Direction::iter() {
      let (dx, dy) = dir.delta();
      result.push((self.0 + dx, self.1 + dy))
    }

    result
  }

  fn toward(&self, direction: Direction) -> TowardIter {
    TowardIter::new(*self, direction)
  }
}

pub struct TowardIter {
  last: (isize, isize),
  direction: Direction,
}

impl TowardIter {
  fn new(coord: (isize, isize), direction: Direction) -> Self {
    Self {
      last: coord,
      direction,
    }
  }
}

impl Iterator for TowardIter {
  type Item = (isize, isize);
  fn next(&mut self) -> Option<Self::Item> {
    let (dx, dy) = self.direction.delta();
    self.last = (self.last.0 + dx, self.last.1 + dy);
    Some(self.last)
  }
}

#[test]
fn test_coords() {
  let p = (1, 3);
  let n: Vec<(isize, isize)> = p.neighbors();
  assert_eq!(
    n,
    vec![
      (1, 4),
      (1, 2),
      (2, 3),
      (0, 3),
      (0, 4),
      (2, 4),
      (0, 2),
      (2, 2),
    ]
  );
}

#[test]
fn test_infinite_coords() {
  let origin = (0, 0);

  let ne = origin
    .toward(Direction::NE)
    .filter(|(x, y)| x % 2 == 0 && y % 2 == 0)
    .take(3)
    .collect::<Vec<_>>();
  assert_eq!(ne, vec![(2, 2), (4, 4), (6, 6)])
}
