use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::RangeInclusive;

pub fn run(input: String, _args: &[String]) {
  let (rules, ticket, nearby) = parse_input(&input);
  {
    let invalid_values = nearby
      .iter()
      .flat_map(|ticket| ticket.invalid_digits(&rules));
    let invalid_values_sum: u32 = invalid_values.sum();

    println!("The ticket scanning error rate is: {}", invalid_values_sum);
  }

  {
    let good_tickets: Vec<_> = nearby
      .iter()
      .cloned()
      .filter(|ticket| ticket.is_valid(&rules))
      .collect();

    let layout = ticket.find_layout(&rules, &good_tickets);
    let departure_fields_product: u64 = layout
      .iter()
      .filter_map(|(idx, field)| match field.starts_with("departure") {
        true => Some(ticket.0[*idx] as u64),
        false => None,
      })
      .product();

    println!(
      "Multiplying the departure fields gives {}",
      departure_fields_product
    );
  }
}

fn parse_input(s: &str) -> (Vec<FieldRule>, Ticket, Vec<Ticket>) {
  let mut rules = vec![];
  let your_ticket;
  let nearby_tickets;

  let mut lines = s.lines().peekable();

  // All lines until the first newline are rules
  while !lines.peek().unwrap().is_empty() {
    rules.push(lines.next().unwrap().parse().unwrap());
  }
  // Consume the newline and the "your ticket" text
  lines.next();
  lines.next();
  // Get the ticket
  your_ticket = lines
    .next()
    .unwrap()
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect();
  // Consume the newline and the "nearby tickets" text
  lines.next();
  lines.next();
  // The rest are tickets
  nearby_tickets = lines
    .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
    .collect();

  (rules, your_ticket, nearby_tickets)
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Ticket(Vec<u32>);

impl Ticket {
  fn invalid_digits(&self, rules: &[FieldRule]) -> Vec<u32> {
    self
      .0
      .iter()
      .filter(|&&n| rules.iter().all(|rule| !rule.is_valid(n)))
      .cloned()
      .collect()
  }

  fn is_valid(&self, rules: &[FieldRule]) -> bool {
    !self
      .0
      .iter()
      .any(|&n| rules.iter().all(|rule| !rule.is_valid(n)))
  }

  fn find_layout(&self, rules: &[FieldRule], others: &[Ticket]) -> Vec<(usize, String)> {
    let mut result = Vec::with_capacity(rules.len());
    let mut available_rules = rules.iter().map(|r| r.name.clone()).collect::<HashSet<_>>();

    let mut candidates = (0..self.0.len())
      .map(|idx| {
        // for each index, find the rules that match all values at that index
        // from all tickets
        rules
          .iter()
          .filter_map(|rule| {
            if others.iter().all(|ticket| rule.is_valid(ticket.0[idx])) {
              Some(rule.name.clone())
            } else {
              None
            }
          })
          .collect::<HashSet<_>>()
      })
      .map(Some)
      .collect::<Vec<_>>();

    while candidates.iter().any(|o| o.is_some()) {
      let (value_idx, rule_name) = {
        let next_item = candidates
          .iter()
          .enumerate()
          .filter_map(|(idx, o)| o.as_ref().map(|set| (idx, set)))
          .find(|(_, set)| {
            let intersection = set.intersection(&available_rules);
            intersection.count() == 1
          })
          .map(|(idx, set)| {
            let intersected = set
              .intersection(&available_rules)
              .cloned()
              .collect::<HashSet<_>>();
            let item = intersected.iter().cloned().next().unwrap();
            (idx, item)
          });

        next_item.unwrap()
      };

      candidates[value_idx].take();
      available_rules.remove(&rule_name);
      result.push((value_idx, rule_name));
    }

    result.sort_unstable_by_key(|(value_idx, _)| *value_idx);
    result
  }
}

impl FromIterator<u32> for Ticket {
  fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Ticket {
    Ticket(iter.into_iter().collect())
  }
}

impl Into<Ticket> for Vec<u32> {
  fn into(self) -> Ticket {
    Ticket(self)
  }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct FieldRule {
  name: String,
  ranges: Vec<RangeInclusive<u32>>,
}

impl FieldRule {
  fn new(name: &str, ranges: Vec<RangeInclusive<u32>>) -> Self {
    Self {
      name: name.to_string(),
      ranges,
    }
  }

  fn is_valid(&self, value: u32) -> bool {
    self.ranges.iter().any(|r| r.contains(&value))
  }
}

impl std::str::FromStr for FieldRule {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref FIELD_RULE_REGEX: Regex =
        Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }

    match FIELD_RULE_REGEX.captures(s) {
      Some(caps) => {
        let name = caps.get(1).unwrap().as_str();
        let d1 = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let d2 = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let d3 = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
        let d4 = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();

        Ok(FieldRule::new(
          name,
          vec![RangeInclusive::new(d1, d2), RangeInclusive::new(d3, d4)],
        ))
      }
      None => Err("Could not parse FieldRule".to_string()),
    }
  }
}

impl std::fmt::Debug for FieldRule {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(
      f,
      "FieldRule<{}, {:?}, {:?}>",
      self.name, self.ranges[0], self.ranges[1]
    )
  }
}

#[test]
fn test_field_rule() {
  let fr = FieldRule::new("departure location", vec![31..=221, 241..=952]);
  assert_eq!(fr.is_valid(30), false);
  assert_eq!(fr.is_valid(31), true);
  assert_eq!(fr.is_valid(221), true);
  assert_eq!(fr.is_valid(222), false);
  assert_eq!(fr.is_valid(240), false);
  assert_eq!(fr.is_valid(241), true);
  assert_eq!(fr.is_valid(952), true);
  assert_eq!(fr.is_valid(953), false);
}

#[test]
fn test_input() {
  use indoc::indoc;
  let input = indoc! {"
    class: 1-3 or 5-7
    row: 6-11 or 33-44
    seat: 13-40 or 45-50

    your ticket:
    7,1,14

    nearby tickets:
    7,3,47
    40,4,50
    55,2,20
    38,6,12
  "}
  .trim();

  let (rules, ticket, nearby) = parse_input(&input);

  assert_eq!(rules[1].is_valid(11), true);
  assert_eq!(ticket, Ticket(vec![7, 1, 14]));
  assert_eq!(nearby[1], Ticket(vec![40, 4, 50]));

  assert_eq!(
    nearby
      .iter()
      .flat_map(|ticket| ticket.invalid_digits(&rules))
      .collect::<Vec<_>>(),
    vec![4, 55, 12]
  );

  let valid_tickets: Vec<_> = nearby
    .iter()
    .cloned()
    .filter(|t| t.is_valid(&rules))
    .collect();

  assert_eq!(
    ticket.find_layout(&rules, &valid_tickets),
    vec![
      (0, "row".to_string()),
      (1, "class".to_string()),
      (2, "seat".to_string())
    ]
  )
}
