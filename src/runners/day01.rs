pub fn run(input: String, _args: &[String]) {
    let mut nums: Vec<u32> = input
        .lines()
        .map(|line| {
            line.parse::<u32>()
                .expect(&format!("Could not parse line: {}", line))
        })
        .collect();

    if let Some((a, b)) = find_elems_adding_to(&mut nums, 2020) {
        println!("{} and {} multiply to give {}", a, b, a * b);
    } else {
        println!("Couldn't find anything adding up to {}!", 2020)
    }
}

fn find_elems_adding_to(v: &mut Vec<u32>, sum: u32) -> Option<(u32, u32)> {
    v.sort();

    let len = v.len();
    let iter1 = v.iter();

    for (idx, &num) in iter1.enumerate() {
        if num > sum || idx == len - 1 {
            continue;
        }

        let iter2 = v.iter().skip(idx + 1);
        for &num2 in iter2 {
            if num + num2 == sum {
                return Some((num, num2));
            }
        }
    }

    None
}

#[test]
fn test_find_array_adds_to() {
    let mut v = vec![1721, 979, 366, 299, 675, 1456];

    assert_eq!(find_elems_adding_to(&mut v, 2020), Some((299, 1721)));
}
