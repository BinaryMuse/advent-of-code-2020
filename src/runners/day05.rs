use std::str::FromStr;

pub fn run(input: String, _args: &[String]) {
    let passes: Vec<BoardingPass> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut ids = passes.iter().map(|pass| pass.seat_id()).collect::<Vec<_>>();
    let max = ids.iter().max().unwrap();
    println!("The boarding pass with the highest seat ID is {}", max);

    ids.sort();
    for window in (&ids[..]).windows(2) {
        if window[0] + 1 != window[1] {
            println!("Your seat ID is {}", window[0] + 1);
            break;
        }
    }
}

struct BoardingPass {
    instructions: String,
}

impl BoardingPass {
    fn new(s: &str) -> Self {
        Self {
            instructions: s.to_string(),
        }
    }

    pub fn row(&self) -> usize {
        let chars: Vec<_> = self.instructions.chars().take(7).collect();
        self.space_partition(128, &chars)
    }

    pub fn column(&self) -> usize {
        let chars: Vec<_> = self.instructions.chars().skip(7).collect();
        self.space_partition(8, &chars)
    }

    pub fn seat_id(&self) -> usize {
        self.row() * 8 + self.column()
    }

    fn space_partition(&self, size: usize, instructions: &[char]) -> usize {
        let mut last = instructions[0];
        let mut lower = 0;
        let mut higher = size - 1;
        let mut split = size / 2;

        let iter = instructions.iter().peekable();
        for &chr in iter {
            last = chr;
            match chr {
                'F' | 'L' => higher -= split,
                _ => lower += split,
            }

            split /= 2;
        }

        if last == 'F' || last == 'L' {
            higher
        } else {
            lower
        }
    }
}

impl FromStr for BoardingPass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BoardingPass::new(s))
    }
}

#[test]
fn test_boarding_pass() {
    let bp1: BoardingPass = "BFFFBBFRRR".parse().unwrap();
    assert_eq!(bp1.row(), 70);
    assert_eq!(bp1.column(), 7);
    assert_eq!(bp1.seat_id(), 567);

    let bp2: BoardingPass = "FFFBBBFRRR".parse().unwrap();
    assert_eq!(bp2.row(), 14);
    assert_eq!(bp2.column(), 7);
    assert_eq!(bp2.seat_id(), 119);

    let bp3: BoardingPass = "BBFFBBFRLL".parse().unwrap();
    assert_eq!(bp3.row(), 102);
    assert_eq!(bp3.column(), 4);
    assert_eq!(bp3.seat_id(), 820);
}
