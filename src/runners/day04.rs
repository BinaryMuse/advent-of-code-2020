use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn run(input: String, _args: &[String]) {
  let passports: Vec<Passport> = input
    .trim()
    .split("\n\n")
    .map(|parts| parts.parse().unwrap())
    .collect();
  let required_count = passports.iter().filter(|p| p.has_required_fields()).count();
  println!("Found {} have required fields", required_count);

  let valid_count = passports.iter().filter(|p| p.is_valid()).count();
  println!("Found {} are fully valid", valid_count);
}

struct Passport {
  data: HashMap<String, String>,
}

impl Passport {
  fn new() -> Self {
    Self {
      data: Default::default(),
    }
  }

  pub fn get(&self, key: &str) -> Option<&str> {
    self.data.get(&key.to_string()).map(|s| s.as_str())
  }

  pub fn set(&mut self, key: &str, val: &str) {
    self.data.insert(key.to_string(), val.to_string());
  }

  pub fn has_required_fields(&self) -> bool {
    lazy_static! {
      static ref PASSPORT_REQUIRED_FIELDS: Vec<&'static str> =
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    }

    PASSPORT_REQUIRED_FIELDS
      .iter()
      .all(|key| self.data.contains_key(&key.to_string()))
  }

  pub fn is_valid(&self) -> bool {
    lazy_static! {
      static ref PASSPORT_HEIGHT_REGEX: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
      static ref PASSPORT_HAIR_REGEX: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
      static ref PASSPORT_EYE_REGEX: Regex =
        Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
      static ref PASSPORT_ID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }

    if !self.has_required_fields() {
      return false;
    }

    let byr = self.get("byr").unwrap();
    let byr_val: u32 = byr.parse().unwrap();
    if byr_val < 1920 || byr_val > 2002 {
      return false;
    }

    let iyr = self.get("iyr").unwrap();
    let iyr_val: u32 = iyr.parse().unwrap();
    if iyr_val < 2010 || iyr_val > 2020 {
      return false;
    }

    let eyr = self.get("eyr").unwrap();
    let eyr_val: u32 = eyr.parse().unwrap();
    if eyr_val < 2020 || eyr_val > 2030 {
      return false;
    }

    let hgt = self.get("hgt").unwrap();
    if !PASSPORT_HEIGHT_REGEX.is_match(hgt) {
      return false;
    }
    let caps = PASSPORT_HEIGHT_REGEX.captures(hgt).unwrap();
    let num: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let unit = caps.get(2).unwrap().as_str();
    match unit {
      "cm" => {
        if num < 150 || num > 193 {
          return false;
        }
      }
      _ => {
        if num < 59 || num > 76 {
          return false;
        }
      }
    }

    let hcl = self.get("hcl").unwrap();
    if !PASSPORT_HAIR_REGEX.is_match(hcl) {
      return false;
    }

    let ecl = self.get("ecl").unwrap();
    if !PASSPORT_EYE_REGEX.is_match(ecl) {
      return false;
    }

    let pid = self.get("pid").unwrap();
    if !PASSPORT_ID_REGEX.is_match(pid) {
      return false;
    }

    true
  }
}

impl FromStr for Passport {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let full = s.split('\n').collect::<Vec<_>>().join(" ");
    let entries: Vec<_> = full
      .split(' ')
      .map(|entry| {
        let mut parts = entry.split(':');
        let name = parts.next().unwrap();
        let value = parts.next().unwrap_or("");
        (name, value)
      })
      .collect();

    let mut passport = Passport::new();
    for (ref name, ref value) in entries.iter() {
      passport.set(name, value);
    }

    Ok(passport)
  }
}

#[test]
fn test_passport() {
  use super::common;

  let input = common::get_input("04_sample").unwrap();
  let passport: Passport =
    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"
      .parse()
      .unwrap();
  assert_eq!(passport.get("ecl"), Some("gry"));
  assert_eq!(passport.get("iyr"), Some("2017"));

  let passports: Vec<Passport> = input
    .split("\n\n")
    .map(|parts| parts.parse().unwrap())
    .collect();
  let num_valid = passports.iter().filter(|p| p.is_valid()).count();
  assert_eq!(num_valid, 2);

  let input = common::get_input("04_sample_valid").unwrap();
  let passports: Vec<Passport> = input
    .split("\n\n")
    .map(|parts| parts.parse().unwrap())
    .collect();
  assert_eq!(passports.iter().all(|p| p.is_valid()), true);

  let input = common::get_input("04_sample_invalid").unwrap();
  let passports: Vec<Passport> = input
    .split("\n\n")
    .map(|parts| parts.parse().unwrap())
    .collect();
  assert_eq!(passports.iter().all(|p| !p.is_valid()), true);
}
