mod common;
mod day01;
mod day02;
mod day03;
mod day04;

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

fn get_runner(day: u32) -> Option<Runner> {
    match day {
        1 => Some(Box::new(day01::run)),
        2 => Some(Box::new(day02::run)),
        3 => Some(Box::new(day03::run)),
        4 => Some(Box::new(day04::run)),
        _ => None,
    }
}
