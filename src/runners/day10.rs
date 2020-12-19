use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;

pub fn run(input: String, _args: &[String]) {
  let mut adapters: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
  // Slice must be sorted
  adapters.sort_unstable();
  let jumps = find_adapter_jumps(&adapters);
  println!(
    "Found {} 1-jumps and {} 3-jumps; multiplied they are {}",
    jumps.0,
    jumps.1,
    jumps.0 * jumps.1
  );

  let arrangements = count_reachable(&adapters);
  println!("Found {} valid arrangements", arrangements);
}

fn find_adapter_jumps(adapters: &[u32]) -> (usize, usize) {
  let mut result: Vec<u32> = vec![0];
  let mut taken = HashSet::new();
  let max = adapters.iter().max().unwrap();
  let final_target = max + 3;

  if do_find_adapter_chain(
    adapters,
    final_target,
    &mut result,
    &mut taken,
    &mut 0,
    false,
  ) {
    let final_chain: Vec<_> = result.iter().chain(iter::once(&final_target)).collect();

    let jumps: Vec<_> = final_chain
      .iter()
      .enumerate()
      .map(|(idx, &num)| {
        if idx == 0 {
          0
        } else {
          let last = final_chain[idx - 1];
          num - last
        }
      })
      .collect();
    let jumps_1 = jumps.iter().filter(|&&n| n == 1).count();
    let jumps_3 = jumps.iter().filter(|&&n| n == 3).count();

    (jumps_1, jumps_3)
  } else {
    panic!("Couldn't find chain of adapters");
  }
}

fn do_find_adapter_chain(
  adapters: &[u32],
  final_target: u32,
  result: &mut Vec<u32>,
  taken: &mut HashSet<usize>,
  count: &mut u32,
  continue_on_find: bool,
) -> bool {
  if result.iter().last().unwrap() + 3 == final_target {
    *count += 1;
    !continue_on_find
  } else {
    let last = result.iter().last().unwrap();
    let candidates = find_candidate_indices(adapters, last, &taken);
    for idx in candidates {
      taken.insert(idx);
      result.push(adapters[idx]);
      let found = do_find_adapter_chain(
        adapters,
        final_target,
        result,
        taken,
        count,
        continue_on_find,
      );
      if found {
        return true;
      } else {
        taken.remove(&idx);
        result.pop();
      }
    }

    false
  }
}

fn find_candidate_indices(adapters: &[u32], previous: &u32, taken: &HashSet<usize>) -> Vec<usize> {
  adapters
    .iter()
    .enumerate()
    .filter(|(idx, &n)| n > *previous && n - *previous <= 3 && !taken.contains(idx))
    .map(|(idx, _)| idx)
    .collect()
}

fn count_reachable(adapters: &[u32]) -> u64 {
  let max = *adapters.last().unwrap();
  let mut cache: HashMap<u32, u64> = HashMap::new();
  let mut adapters_with_end = adapters.to_vec();
  adapters_with_end.push(max + 3);
  do_count_reachable(&0, &adapters_with_end, max, &mut cache)
}

#[allow(clippy::map_entry)]
fn do_count_reachable(
  current_end: &u32,
  available_adapters: &[u32],
  max: u32,
  cache: &mut HashMap<u32, u64>,
) -> u64 {
  if *current_end == max {
    // We're at the end, it's one hop to the device
    return 1;
  }

  let mut count = 0;
  for i in 1..=3 {
    let new_end = current_end + i;
    if available_adapters.contains(&new_end) {
      let remaining = available_adapters.iter().filter(|&&n| n > new_end);
      if !cache.contains_key(&new_end) {
        let val = do_count_reachable(
          &new_end,
          &remaining.cloned().collect::<Vec<_>>(),
          max,
          cache,
        );
        cache.insert(new_end, val);
      }
      count += cache.get(&new_end).unwrap();
    }
  }

  count
}

#[test]
fn test_adapter() {
  let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
  adapters.sort_unstable();

  let jumps = find_adapter_jumps(&adapters);
  assert_eq!(jumps, (7, 5));

  let valid_count = count_reachable(&adapters);
  assert_eq!(valid_count, 8);
}
