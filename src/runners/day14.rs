use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn run(input: String, _args: &[String]) {
  {
    let instructions = parse_input(&input);
    let mut comp = DockingComputer::new();
    comp.process_instructions_v1(&instructions);
    let sum = comp.memory_sum();
    println!("The sum of values in memory for v1 is {}", sum);
  }

  {
    let instructions = parse_input(&input);
    let mut comp = DockingComputer::new();
    comp.process_instructions_v2(&instructions);
    let sum = comp.memory_sum();
    println!("The sum of values in memory for v2 is {}", sum);
  }
}

fn parse_input(input: &str) -> Vec<Instruction> {
  input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug, Clone)]
struct DockingComputer {
  mask: Option<Mask>,
  mem: HashMap<u64, u64>,
}

impl DockingComputer {
  fn new() -> Self {
    Self {
      mask: None,
      mem: Default::default(),
    }
  }

  fn memory_sum(&self) -> u64 {
    self.mem.values().sum()
  }

  fn process_instructions_v1(&mut self, instructions: &[Instruction]) {
    for inst in instructions.iter() {
      self.process_instruction_v1(inst);
    }
  }

  fn process_instruction_v1(&mut self, instruction: &Instruction) {
    match instruction {
      Instruction::SetValue(address, value) => {
        self.mem.insert(
          *address,
          self
            .mask
            .as_ref()
            .expect("No mask found!")
            .apply_value(*value),
        );
      }
      Instruction::UpdateMask(mask) => {
        self.mask = Some(mask.clone());
      }
    }
  }

  fn process_instructions_v2(&mut self, instructions: &[Instruction]) {
    for inst in instructions.iter() {
      self.process_instruction_v2(inst);
    }
  }

  fn process_instruction_v2(&mut self, instruction: &Instruction) {
    match instruction {
      Instruction::SetValue(address, value) => {
        for addr in self
          .mask
          .as_ref()
          .expect("No mask found!")
          .apply_address(*address)
        {
          self.mem.insert(addr, *value);
        }
      }
      Instruction::UpdateMask(mask) => {
        self.mask = Some(mask.clone());
      }
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MaskBit {
  One,
  Zero,
  Floating,
}

#[derive(Debug, Clone, PartialEq)]
struct Mask {
  overrides: HashMap<usize, MaskBit>,
}

impl Mask {
  fn new() -> Self {
    Self {
      overrides: Default::default(),
    }
  }

  fn set_override(&mut self, idx: usize, value: MaskBit) {
    self.overrides.insert(idx, value);
  }

  fn apply_value(&self, num: u64) -> u64 {
    let mut num = num;
    for (&idx, &val) in self.overrides.iter() {
      match val {
        MaskBit::One => num |= 1 << idx,
        MaskBit::Zero => num &= !(1 << idx),
        _ => {}
      }
    }

    num
  }

  fn apply_address(&self, address: u64) -> Vec<u64> {
    let mut address = address;

    // All bits with a mask of "1" get set to "1"
    for (&idx, _) in self
      .overrides
      .iter()
      .filter(|(_, &val)| val == MaskBit::One)
    {
      address |= 1 << idx;
    }

    // All bits with a floating mask become both "1" and "0"
    let floating_bits: Vec<_> = self
      .overrides
      .iter()
      .filter_map(|(&idx, &val)| match val {
        MaskBit::Floating => Some(idx),
        _ => None,
      })
      .collect();

    let mut result = vec![];
    self.find_floating_candidates(address, &floating_bits, &mut result);
    result.sort_unstable();
    result
  }

  fn find_floating_candidates(&self, base: u64, bits: &[usize], result: &mut Vec<u64>) {
    if let Some(idx) = bits.get(0) {
      let base_one = base | 1 << idx;
      let base_zero = base & !(1 << idx);

      self.find_floating_candidates(base_one, &bits[1..], result);
      self.find_floating_candidates(base_zero, &bits[1..], result);
    } else {
      result.push(base);
    }
  }
}

impl FromStr for Mask {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut result = Self::new();

    for (idx, chr) in s.chars().rev().enumerate() {
      let bit = match chr {
        '1' => MaskBit::One,
        '0' => MaskBit::Zero,
        _ => MaskBit::Floating,
      };

      result.set_override(idx, bit);
    }

    Ok(result)
  }
}

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
  UpdateMask(Mask),
  SetValue(u64, u64),
}

impl FromStr for Instruction {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref INSTRUCTION_SET_REGEX: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
      static ref INSTRUCTION_MASK_REGEX: Regex = Regex::new(r"mask = (.*)").unwrap();
    }

    if INSTRUCTION_SET_REGEX.is_match(s) {
      let caps = INSTRUCTION_SET_REGEX.captures(s).unwrap();
      let address: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
      let value: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
      Ok(Instruction::SetValue(address, value))
    } else {
      let caps = INSTRUCTION_MASK_REGEX.captures(s).unwrap();
      let mask: Mask = caps.get(1).unwrap().as_str().parse().unwrap();
      Ok(Instruction::UpdateMask(mask))
    }
  }
}

#[test]
fn test_mask() {
  let input = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
  let mask: Mask = input.parse().unwrap();

  assert_eq!(mask.apply_value(11), 73);
  assert_eq!(mask.apply_value(101), 101);
  assert_eq!(mask.apply_value(0), 64);

  let input = "000000000000000000000000000000X1001X";
  let mask: Mask = input.parse().unwrap();

  assert_eq!(mask.apply_address(42), vec![26, 27, 58, 59]);

  let input = "00000000000000000000000000000000X0XX";
  let mask: Mask = input.parse().unwrap();

  assert_eq!(mask.apply_address(26), vec![16, 17, 18, 19, 24, 25, 26, 27]);
}

#[test]
fn test_instruction() {
  let inst: Instruction = "mem[8] = 11".parse().unwrap();
  assert_eq!(inst, Instruction::SetValue(8, 11));

  let mask: Mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();
  let inst: Instruction = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
    .parse()
    .unwrap();
  assert_eq!(inst, Instruction::UpdateMask(mask));
}

#[test]
fn test_part_1() {
  let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
  let instructions = parse_input(input);
  let mut comp = DockingComputer::new();

  comp.process_instructions_v1(&instructions);
  assert_eq!(comp.memory_sum(), 165);
}

#[test]
fn test_part_2() {
  let input = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
  let instructions = parse_input(input);
  let mut comp = DockingComputer::new();

  comp.process_instructions_v2(&instructions);
  assert_eq!(comp.memory_sum(), 208);
}

#[test]
fn test_bits() {
  let mut num: u64 = 9;
  num &= !(1 << 0);
  assert_eq!(num, 8);
  num |= 1 << 1;
  assert_eq!(num, 10);
}
