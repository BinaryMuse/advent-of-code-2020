use regex::Regex;
use std::fmt::Display;
use std::str::FromStr;

pub fn run(input: String, _args: &[String]) {
    let policies_and_passwords = input
        .lines()
        .map(|s| {
            let mut split = s.split(": ");
            let left = split.next().unwrap();
            let right = split.next().unwrap();
            let policy = left.parse::<PasswordPolicy>().unwrap();
            (policy, right)
        })
        .collect::<Vec<_>>();

    let num_passing_old = policies_and_passwords
        .iter()
        .filter(|(policy, pass)| policy.check_old(pass))
        .count();

    let num_passing_new = policies_and_passwords
        .iter()
        .filter(|(policy, pass)| policy.check_new(pass))
        .count();

    println!(
        "{} passwords match based on the old policy",
        num_passing_old
    );
    println!(
        "{} passwords match based on the new policy",
        num_passing_new
    );
}

struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
}

impl PasswordPolicy {
    fn new(min: usize, max: usize, letter: char) -> Self {
        Self { min, max, letter }
    }

    pub fn check_old(&self, s: &str) -> bool {
        let count = s.chars().filter(|&c| c == self.letter).count();
        return count >= self.min && count <= self.max;
    }

    pub fn check_new(&self, s: &str) -> bool {
        let chars = s.chars().collect::<Vec<_>>();
        let left_char = chars[self.min - 1];
        let right_char = chars[self.max - 1];

        (left_char == self.letter || right_char == self.letter) && left_char != right_char
    }
}

impl FromStr for PasswordPolicy {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref PASS_POLICY_RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z])").unwrap();
        }

        match PASS_POLICY_RE.captures(s) {
            Some(cap) => {
                let low = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let high = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let letter = cap.get(3).unwrap().as_str().parse::<char>().unwrap();
                Ok(PasswordPolicy::new(low, high, letter))
            }
            _ => Err(format!("Unable to parse password policy: {}", s)),
        }
    }
}

impl Display for PasswordPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}-{} {}", self.min, self.max, self.letter)
    }
}

#[test]
fn test_old_password_policy() {
    let policy: PasswordPolicy = "2-5 c".parse().unwrap();
    assert_eq!(policy.check_old("cc"), true);
    assert_eq!(policy.check_old("ccccc"), true);
    assert_eq!(policy.check_old("cbc"), true);
    assert_eq!(policy.check_old("c"), false);
    assert_eq!(policy.check_old("cccccc"), false);

    let policy2: PasswordPolicy = "1-3 a".parse().unwrap();
    assert_eq!(policy2.check_old("abcde"), true);
    assert_eq!(policy2.check_old("bcdef"), false);
}

#[test]
fn test_new_password_policy() {
    let policy: PasswordPolicy = "2-5 c".parse().unwrap();
    assert_eq!(policy.check_new("ccaba"), true);
    assert_eq!(policy.check_new("abfec"), true);
    assert_eq!(policy.check_new("ccccb"), true);
    assert_eq!(policy.check_new("acbec"), false);
    assert_eq!(policy.check_new("abcde"), false);
}
