mod common;
mod day01;

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
        _ => None,
    }
}
