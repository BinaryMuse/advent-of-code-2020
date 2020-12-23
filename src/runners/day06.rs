use std::collections::HashSet;

pub fn run(input: String, _args: &[String]) {
  let groups = groups_from_input(&input);
  let questions_per_group: Vec<_> = groups.iter().map(|g| any_questions_for_group(g)).collect();
  let sum = questions_per_group.iter().fold(0, |acc, hs| acc + hs.len());
  println!("Sum of any questions from all groups is {}", sum);

  let questions_per_group: Vec<_> = groups.iter().map(|g| all_questions_for_group(g)).collect();
  let sum = questions_per_group.iter().fold(0, |acc, hs| acc + hs.len());
  println!("Sum of all questions from all groups is {}", sum);
}

fn groups_from_input(input: &str) -> Vec<&str> {
  input.trim().split("\n\n").collect()
}

fn any_questions_for_group(group: &str) -> HashSet<char> {
  let mut hs: HashSet<char> = HashSet::new();

  for line in group.lines() {
    for char in line.chars() {
      hs.insert(char);
    }
  }

  hs
}

fn all_questions_for_group(group: &str) -> HashSet<char> {
  let hashsets: Vec<_> = group
    .lines()
    .map(|line| {
      let mut hs = HashSet::new();
      for chr in line.chars() {
        hs.insert(chr);
      }

      hs
    })
    .collect();
  let mut iter = hashsets.iter();
  let first = iter.next().unwrap().clone();
  iter.fold(first, |acc, hs| acc.intersection(hs).cloned().collect())
}
