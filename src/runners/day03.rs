use std::str::FromStr;

pub fn run(input: String, _args: &[String]) {
  let grid: InfiniteHorizontalGrid = input.parse().expect("Couldn't parse grid");
  let trees_hit = grid.taken_on_slope(3, 1);
  println!("Moving by (3, 2) you'll hit {} trees", trees_hit);

  let alt1 = grid.taken_on_slope(1, 1);
  let alt2 = grid.taken_on_slope(5, 1);
  let alt3 = grid.taken_on_slope(7, 1);
  let alt4 = grid.taken_on_slope(1, 2);
  println!(
    "The other slopes multiplied together is {}",
    alt1 * alt2 * alt3 * alt4 * trees_hit
  );
}

struct InfiniteHorizontalGrid {
  width: usize,
  height: usize,
  lines: Vec<Vec<bool>>,
}

impl InfiniteHorizontalGrid {
  pub fn value_at(&self, row: usize, col: usize) -> Option<bool> {
    if row >= self.height {
      return None;
    }

    let mut wrapped_col = col;

    while wrapped_col >= self.width {
      wrapped_col -= self.width;
    }

    Some(self.lines[row][wrapped_col])
  }

  pub fn taken_on_slope(&self, dx: i64, dy: i64) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut count: u64 = 0;

    while let Some(tree_present) = self.value_at(y, x) {
      if tree_present {
        count += 1;
      }

      x += dx as usize;
      y += dy as usize;
    }

    count
  }
}

impl FromStr for InfiniteHorizontalGrid {
  type Err = String;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    let lines: Vec<Vec<bool>> = s
      .lines()
      .map(|line| line.chars().map(|c| c == '#').collect())
      .collect();

    Ok(Self {
      width: lines[0].len(),
      height: lines.len(),
      lines,
    })
  }
}

#[test]
fn test_horizontal_grid() {
  use super::common;
  let input = common::get_input("03_sample").unwrap();
  let grid: InfiniteHorizontalGrid = input.parse().unwrap();
  assert_eq!(grid.value_at(0, 0), Some(false));
  assert_eq!(grid.value_at(1, 0), Some(true));
  assert_eq!(grid.value_at(1, 0), Some(true));
  assert_eq!(grid.value_at(11, 0), None);

  assert_eq!(grid.taken_on_slope(1, 1), 2);
  assert_eq!(grid.taken_on_slope(3, 1), 7);
  assert_eq!(grid.taken_on_slope(5, 1), 3);
  assert_eq!(grid.taken_on_slope(7, 1), 4);
  assert_eq!(grid.taken_on_slope(1, 2), 2);
}
