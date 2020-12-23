use super::coords::Direction;
use std::str::FromStr;

pub fn run(input: String, _args: &[String]) {
  let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();
  let mut ship = Ship::new();
  for instr in instructions.iter() {
    ship.navigate(instr);
  }

  println!(
    "Manhattan distance: {}",
    manhattan_distance_from_origin(ship.pos)
  );

  let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();
  let mut ship = Ship::new();
  for instr in instructions.iter() {
    ship.navigate_part_2(instr);
  }

  println!(
    "Manhattan distance with improved navigation: {}",
    manhattan_distance_from_origin(ship.pos)
  );
}

#[derive(Debug)]
struct Ship {
  pos: (isize, isize),
  waypoint: (isize, isize),
  facing: Direction,
}

impl Ship {
  fn new() -> Self {
    Ship {
      pos: (0, 0),
      waypoint: (10, 1),
      facing: Direction::E,
    }
  }

  fn navigate(&mut self, instr: &Instruction) {
    match instr {
      Instruction::N(amount) => self.pos.1 += amount,
      Instruction::S(amount) => self.pos.1 -= amount,
      Instruction::E(amount) => self.pos.0 += amount,
      Instruction::W(amount) => self.pos.0 -= amount,
      Instruction::L(degrees) => {
        let count = degrees / 90;
        for _ in 0..count {
          self.facing = self.facing.left_90();
        }
      }
      Instruction::R(degrees) => {
        let count = degrees / 90;
        for _ in 0..count {
          self.facing = self.facing.right_90();
        }
      }
      Instruction::F(units) => {
        let (x, y) = self.facing.delta();
        self.pos = (self.pos.0 + x * units, self.pos.1 + y * units);
      }
    }
  }

  fn navigate_part_2(&mut self, instr: &Instruction) {
    match instr {
      Instruction::N(amount) => self.waypoint.1 += amount,
      Instruction::S(amount) => self.waypoint.1 -= amount,
      Instruction::E(amount) => self.waypoint.0 += amount,
      Instruction::W(amount) => self.waypoint.0 -= amount,
      Instruction::L(degrees) => {
        let count = degrees / 90;
        for _ in 0..count {
          let (x, y) = (-self.waypoint.1, self.waypoint.0);
          self.waypoint = (x, y);
        }
      }
      Instruction::R(degrees) => {
        let count = degrees / 90;
        for _ in 0..count {
          let (x, y) = (self.waypoint.1, -self.waypoint.0);
          self.waypoint = (x, y);
        }
      }
      Instruction::F(units) => {
        let dx = self.waypoint.0 * units;
        let dy = self.waypoint.1 * units;
        let (x, y) = self.pos;
        self.pos = (x + dx, y + dy);
      }
    }
  }
}

#[derive(Debug)]
enum Instruction {
  N(isize),
  S(isize),
  E(isize),
  W(isize),
  L(isize),
  R(isize),
  F(isize),
}

impl FromStr for Instruction {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chars = s.chars();
    let kind = chars.next().unwrap();
    let rest: String = chars.collect();
    let num: isize = rest.parse().unwrap();

    match kind {
      'N' => Ok(Instruction::N(num)),
      'S' => Ok(Instruction::S(num)),
      'E' => Ok(Instruction::E(num)),
      'W' => Ok(Instruction::W(num)),
      'L' => Ok(Instruction::L(num)),
      'R' => Ok(Instruction::R(num)),
      'F' => Ok(Instruction::F(num)),
      _ => Err(format!("Could not parse instruction: {}", s)),
    }
  }
}

fn manhattan_distance_from_origin(coord: (isize, isize)) -> isize {
  coord.0.abs() + coord.1.abs()
}

#[test]
fn test_ship() {
  let input = "F10\nN3\nF7\nR90\nF11";
  let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

  let mut ship = Ship::new();
  for instr in instructions.iter() {
    ship.navigate(instr);
  }

  assert_eq!(ship.pos, (17, -8));
  assert_eq!(manhattan_distance_from_origin(ship.pos), 25);

  let mut ship = Ship::new();
  for instr in instructions.iter() {
    ship.navigate_part_2(instr);
  }

  assert_eq!(ship.pos, (214, -72));
  assert_eq!(manhattan_distance_from_origin(ship.pos), 286);
}
