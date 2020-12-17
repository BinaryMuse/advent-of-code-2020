use std::collections::HashMap;
use std::collections::VecDeque;

pub fn run(input: String, _args: &[String]) {
  let nums: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
  let mut window = SumWindow::new(nums.clone(), 25, 25);
  let invalid = window.find_invalid_number().unwrap();
  println!("The first invalid number is {}", invalid);

  let weakness = find_encryption_weakness(&nums, invalid).unwrap();
  println!("Encryption weakness value is {}", weakness);
}

struct SumWindow {
  values: Vec<u64>,
  window: VecDeque<u64>,
  cache: HashMap<u64, usize>,
  window_size: usize,
}

impl SumWindow {
  fn new(values: Vec<u64>, preamble_size: usize, window_size: usize) -> Self {
    let mut instance = Self {
      values: values.iter().cloned().skip(preamble_size).collect(),
      window: VecDeque::with_capacity(window_size),
      cache: Default::default(),
      window_size,
    };

    for &num in values.iter().take(preamble_size) {
      instance.add_to_window(num);
    }

    instance
  }

  pub fn find_invalid_number(&mut self) -> Option<u64> {
    for &num in self.values.clone().iter() {
      if !self.is_valid(num) {
        return Some(num);
      }

      self.add_to_window(num);
    }

    None
  }

  fn add_to_window(&mut self, num: u64) {
    if self.window.len() == self.window_size {
      self.drop_leading();
    }

    for &existing in self.window.iter() {
      let sum = existing + num;
      self.cache.entry(sum).and_modify(|n| *n += 1).or_insert(1);
    }

    self.window.push_back(num);
  }

  fn drop_leading(&mut self) {
    let leading = self.window.pop_front().unwrap();
    for &num in self.window.iter() {
      let sum = num + leading;
      self.cache.entry(sum).and_modify(|n| *n -= 1);
    }
  }

  fn is_valid(&self, num: u64) -> bool {
    matches!(self.cache.get(&num), Some(&v) if v > 0)
  }
}

fn find_encryption_weakness(nums: &[u64], target: u64) -> Option<u64> {
  let mut working_set: VecDeque<u64> = Default::default();
  let mut sum: u64 = 0;

  for &n in nums.iter() {
    working_set.push_back(n);
    sum += n;

    while sum > target {
      sum -= working_set.pop_front().unwrap();
    }

    if sum == target && working_set.len() >= 2 {
      let max = working_set.iter().max().unwrap();
      let min = working_set.iter().min().unwrap();
      return Some(min + max);
    }
  }

  None
}

#[test]
fn test_xmas() {
  let nums = vec![
    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
  ];

  let mut window = SumWindow::new(nums.clone(), 5, 5);
  let not_valid = window.find_invalid_number();
  assert_eq!(not_valid, Some(127));

  let weakness = find_encryption_weakness(&nums, 127);
  assert_eq!(weakness, Some(62));
}
