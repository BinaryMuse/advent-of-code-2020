use std::str::FromStr;

pub fn run(input: String, _args: &[String]) {
  let schedule: Schedule = input.parse().unwrap();
  let (earliest_bus, mins_waited) = schedule.find_earliest_bus().unwrap();
  println!(
    "The earliest bus is bus # {}, waiting {} minutes. Multiplied they are {}",
    earliest_bus,
    mins_waited,
    earliest_bus * mins_waited
  );

  println!(
    "The contest solution is {}",
    schedule.find_contest_solution().unwrap()
  );
}

struct Schedule {
  earliest: u64,
  busses: Vec<Option<u64>>,
}

impl Schedule {
  fn find_earliest_bus(&self) -> Option<(u64, u64)> {
    let active_busses = self.busses.iter().filter_map(|&bus| bus);

    let mins_wait_per_bus = active_busses.clone().map(|bus| {
      let minutes_ago = self.earliest % bus;
      let mins_waited = if minutes_ago == 0 {
        0
      } else {
        bus - minutes_ago
      };

      (bus, mins_waited)
    });

    mins_wait_per_bus.min_by_key(|(_, mins)| *mins)
  }

  fn find_contest_solution(&self) -> Option<u64> {
    let scheduled_busses: Vec<_> = self
      .busses
      .iter()
      .enumerate()
      .filter(|(_, o)| o.is_some())
      .map(|(i, o)| (i, o.unwrap()))
      .collect();

    let mut time = 0;
    let mut jump_size = None;

    for window in scheduled_busses.windows(2) {
      let &(prev_idx, prev_minutes) = window.get(0).unwrap();
      let &(next_idx, next_minutes) = window.get(1).unwrap();

      let mut jump_iter = IncrIter::new(*jump_size.get_or_insert(prev_minutes), time);
      time = jump_iter
        .find(|&candidate| {
          let matches_prev = (candidate + prev_idx as u64) % prev_minutes == 0;
          let matches_next = (candidate + next_idx as u64) % next_minutes == 0;
          matches_prev && matches_next
        })
        .unwrap();

      jump_size = jump_size.map(|n| n * next_minutes);
    }

    Some(time)
  }
}

impl FromStr for Schedule {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut lines = s.lines();
    let earliest: u64 = lines.next().unwrap().parse().unwrap();
    let busses: Vec<Option<u64>> = lines
      .next()
      .unwrap()
      .split(',')
      .map(|s| s.parse().ok())
      .collect();

    Ok(Self { earliest, busses })
  }
}

struct IncrIter {
  incr: u64,
  last: u64,
}

impl IncrIter {
  fn new(incr: u64, start: u64) -> Self {
    Self { incr, last: start }
  }
}

impl Iterator for IncrIter {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    self.last += self.incr;
    Some(self.last)
  }
}

#[test]
fn test_bus_times() {
  let input = "939\n7,13,x,x,59,x,31,19";
  let schedule: Schedule = input.parse().unwrap();
  assert_eq!(schedule.earliest, 939);
  assert_eq!(
    schedule.busses,
    vec![
      Some(7),
      Some(13),
      None,
      None,
      Some(59),
      None,
      Some(31),
      Some(19)
    ]
  );

  let mut iter = IncrIter::new(13, 1);
  assert_eq!(iter.next(), Some(14));
  assert_eq!(iter.next(), Some(27));

  assert_eq!(schedule.find_earliest_bus(), Some((59, 5)));
  assert_eq!(schedule.find_contest_solution(), Some(1068781));
}
