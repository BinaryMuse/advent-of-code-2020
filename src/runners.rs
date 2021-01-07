mod common;
mod coords;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod grid;

type Runner = Box<dyn Fn(String, &[String])>;

pub fn run_day(day: u32, args: &[String]) {
  let runner = get_runner(day).unwrap_or_else(|| panic!("No runner found for day {}", day));
  let input_file = if day < 10 {
    format!("0{}", day)
  } else {
    format!("{}", day)
  };
  let input = common::get_input(&input_file).expect("Couldn't find input file");
  runner(input, args);
}

#[allow(clippy::zero_prefixed_literal)]
fn get_runner(day: u32) -> Option<Runner> {
  match day {
    01 => Some(Box::new(day01::run)),
    02 => Some(Box::new(day02::run)),
    03 => Some(Box::new(day03::run)),
    04 => Some(Box::new(day04::run)),
    05 => Some(Box::new(day05::run)),
    06 => Some(Box::new(day06::run)),
    07 => Some(Box::new(day07::run)),
    08 => Some(Box::new(day08::run)),
    09 => Some(Box::new(day09::run)),
    10 => Some(Box::new(day10::run)),
    11 => Some(Box::new(day11::run)),
    12 => Some(Box::new(day12::run)),
    13 => Some(Box::new(day13::run)),
    14 => Some(Box::new(day14::run)),
    15 => Some(Box::new(day15::run)),
    16 => Some(Box::new(day16::run)),
    _ => None,
  }
}
