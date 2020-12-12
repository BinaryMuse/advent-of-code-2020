use itertools::Itertools;

pub fn run(input: String, _args: &[String]) {
    let nums: Vec<u32> = input
        .lines()
        .map(|line| {
            line.parse::<u32>()
                .expect(&format!("Could not parse line: {}", line))
        })
        .collect();

    let two_elems =
        find_elems_adding_to(&nums, 2, 2020).expect("Couldn't find two items adding to 2020");
    if let [a, b] = two_elems[..] {
        println!("{} and {} multiply to give {}", a, b, a * b);
    }

    let three_elems =
        find_elems_adding_to(&nums, 3, 2020).expect("Couldn't find three items adding to 2020");
    if let [a, b, c] = three_elems[..] {
        println!("{}, {}, and {} multiply to give {}", a, b, c, a * b * c);
    }
}

fn find_elems_adding_to(v: &[u32], count: usize, sum: u32) -> Option<Vec<u32>> {
    let combinations = v.iter().cloned().combinations(count);
    for combo in combinations {
        if combo.iter().fold(0, |acc, &x| acc + x) == sum {
            return Some(combo);
        }
    }

    None
}

#[test]
fn test_find_array_adds_to() {
    let v = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_elems_adding_to(&v, 2, 2020), Some(vec![1721, 299]));
    assert_eq!(find_elems_adding_to(&v, 3, 2020), Some(vec![979, 366, 675]));
}
