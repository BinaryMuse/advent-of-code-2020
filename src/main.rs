#[macro_use]
extern crate lazy_static;
mod runners;

fn main() -> Result<(), std::io::Error> {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    eprintln!("Usage: {} <day_num> [args...]", args[0]);
    std::process::exit(1);
  }

  let day: u32 = args[1].parse::<u32>().expect("Could not parse day number");
  let remaining = &args[2..];
  runners::run_day(day, remaining);

  Ok(())
}
