use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

pub fn run(input: String, _args: &[String]) {
    let mut rules = Rules::new();
    for bag in input.lines().map(|line| line.parse::<Bag>().unwrap()) {
        rules.add(&bag);
    }

    let contains_shiny_gold_bags = rules.find_contains("shiny gold").len();
    println!(
        "Shiny gold bags can be contained by {} total bags.",
        contains_shiny_gold_bags
    );

    let shiny_gold_bag_contains = rules.get_child_count("shiny gold", 0);
    println!(
        "Shiny gold bags contain a total of {} other bags",
        shiny_gold_bag_contains
    );
}

#[derive(Debug, Clone, PartialEq)]
struct Bag {
    color: String,
    children: Option<Vec<(usize, String)>>,
    parents: Vec<String>,
}

impl Bag {
    fn new(color: &str, children: Option<Vec<(usize, String)>>) -> Self {
        Self {
            color: color.to_string(),
            children,
            parents: vec![],
        }
    }
}

impl FromStr for Bag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RULES_LINE_REGEX: regex::Regex =
                Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
            static ref RULES_CLAUSE_REGEX: regex::Regex =
                Regex::new(r"(\d+) (.*) bag(?:s)?").unwrap();
        }

        let caps = RULES_LINE_REGEX.captures(s).unwrap();
        let bag_color = caps.get(1).unwrap().as_str();
        let rule_clauses = caps.get(2).unwrap().as_str();

        match rule_clauses {
            "no other bags" => Ok(Bag::new(bag_color, Some(vec![]))),
            clause => {
                let children: Vec<_> = clause
                    .split(", ")
                    .map(|clause| {
                        let clause_caps = RULES_CLAUSE_REGEX.captures(clause).unwrap();
                        let num = clause_caps
                            .get(1)
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap();
                        let bag_color = clause_caps.get(2).unwrap().as_str();

                        (num, bag_color.to_string())
                    })
                    .collect();

                Ok(Bag::new(bag_color, Some(children)))
            }
        }
    }
}

struct Rules {
    bags: HashMap<String, Bag>,
}

impl Rules {
    fn new() -> Self {
        Self {
            bags: Default::default(),
        }
    }

    pub fn add(&mut self, bag: &Bag) {
        for (_size, color) in bag.children.as_ref().unwrap().iter() {
            self.tap_child(color, &bag.color);
        }

        // If bag already exists in the map, it's because it
        // was defined implicitly via a parent and the parent added
        // refs to itself inside the `parents` vector, but set
        // `children` to `None`.
        let mut inserted_bag = self.bags.entry(bag.color.clone()).or_insert_with(|| bag.clone());
        inserted_bag.children = bag.children.clone();
    }

    pub fn find_contains(&self, color: &str) -> HashSet<String> {
        let mut all_parents = HashSet::new();
        self.iterate_parents(color, &mut all_parents);

        all_parents
    }

    pub fn get_child_count(&self, color: &str, i: usize) -> usize {
        let bag = self.bags.get(&color.to_string()).unwrap();

        match &bag.children {
            Some(children) => children.iter().fold(0, |acc, (size, child)| {
                acc + size + self.get_child_count(child, i + 1) * size
            }),
            None => 0,
        }
    }

    fn tap_child(&mut self, color: &str, parent: &str) {
        let child_bag = self
            .bags
            .entry(color.to_string())
            .or_insert_with(|| Bag::new(color, None));
        child_bag.parents.push(parent.to_string());
    }

    fn iterate_parents(&self, color: &str, seen: &mut HashSet<String>) {
        let bag = self.bags.get(&color.to_string()).unwrap();
        for parent_color in bag.parents.iter() {
            if !seen.contains(parent_color) {
                seen.insert(parent_color.clone());
                self.iterate_parents(parent_color, seen);
            }
        }
    }
}

#[test]
fn test_rules() {
    use super::common;

    let input = common::get_input("07_sample").unwrap();
    let mut rules = Rules::new();
    for bag in input.lines().map(|line| line.parse::<Bag>().unwrap()) {
        rules.add(&bag);
    }

    let contains_shiny_gold_bags = rules.find_contains("shiny gold").len();
    assert_eq!(contains_shiny_gold_bags, 4);
}
