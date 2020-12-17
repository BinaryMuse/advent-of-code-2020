use std::collections::HashSet;
use std::str::FromStr;

pub fn run(input: String, _args: &[String]) {
  let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

  let mut console = Console::new(instructions.clone());
  console.run();
  println!(
    "Just before an infinite loop, the accumumlator is: {}",
    console.accumulator
  );

  let swappable_instructions: Vec<_> = instructions
    .iter()
    .enumerate()
    .filter(|(_, ref instr)| matches!(instr.operation, Operation::Jmp | Operation::Nop))
    .map(|(idx, _)| idx)
    .collect();

  // Not a fan of all the cloning but it's simple, it works, and it's fast
  // enough.
  //
  // $ time cargo run --release -- 8 cargo run --release -- 8  0.07s user 0.01s
  //   0.07s user 0.01s system 97% cpu 0.090 total
  for &swappable_idx in swappable_instructions.iter() {
    let mut instructions = instructions.clone();
    let instr = &instructions[swappable_idx];
    let alt = alternate_instruction(&instr);
    instructions[swappable_idx] = alt;
    let mut console = Console::new(instructions);
    if let RunResult::Complete = console.run() {
      println!("Fixed the program! Acc is {}", console.accumulator);
      return;
    }
  }

  println!("Couldn't find a fix to the console...");
}

fn alternate_instruction(instr: &Instruction) -> Instruction {
  let mut clone = instr.clone();
  let new_op = match clone.operation {
    Operation::Jmp => Operation::Nop,
    Operation::Nop => Operation::Jmp,
    _ => panic!("Unable to find alternate instruction for {:?}", clone),
  };

  clone.operation = new_op;
  clone
}

struct Console {
  instructions: Vec<Instruction>,
  accumulator: i32,
  ran_instructions: HashSet<usize>,
  next_instruction: usize,
}

enum RunResult {
  Complete,
  InfiniteLoop,
}

impl Console {
  fn new(instructions: Vec<Instruction>) -> Self {
    Self {
      instructions,
      accumulator: 0,
      ran_instructions: Default::default(),
      next_instruction: 0,
    }
  }

  pub fn run(&mut self) -> RunResult {
    if self.ran_instructions.contains(&self.next_instruction) {
      return RunResult::InfiniteLoop;
    }

    if self.next_instruction == self.instructions.len() {
      return RunResult::Complete;
    }

    self.ran_instructions.insert(self.next_instruction);
    let instr = &self.instructions[self.next_instruction];
    match instr.operation {
      Operation::Acc => {
        self.accumulator += instr.argument;
        self.next_instruction += 1;
      }
      Operation::Jmp => {
        self.next_instruction = ((self.next_instruction as i32) + instr.argument) as usize;
      }
      Operation::Nop => {
        self.next_instruction += 1;
      }
    }

    self.run()
  }
}

#[derive(Debug, Clone)]
struct Instruction {
  operation: Operation,
  argument: i32,
}

impl Instruction {
  fn new(operation: Operation, argument: i32) -> Self {
    Self {
      operation,
      argument,
    }
  }
}

impl FromStr for Instruction {
  type Err = String;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    let mut parts = s.split(' ');
    let op = parts.next().unwrap().parse().unwrap();
    let arg = parts.next().unwrap().parse().unwrap();
    Ok(Self::new(op, arg))
  }
}

#[derive(Debug, Clone)]
enum Operation {
  Acc,
  Jmp,
  Nop,
}

impl FromStr for Operation {
  type Err = String;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      "acc" => Ok(Self::Acc),
      "jmp" => Ok(Self::Jmp),
      "nop" => Ok(Self::Nop),
      _ => Err(format!("Unknown operation: {}", s)),
    }
  }
}
