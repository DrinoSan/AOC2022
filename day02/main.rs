use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    // Der Array [ Rock < Paper < Scissors ]
    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    fn our_score(self) -> usize {
        self.ours.inherent_points() + self.outcome().inherent_points()
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <theirs>SP<ours>EOF, got {s:?}"));
        };

        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let rounds: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Round>())
        .collect::<Result<_, _>>()?;

    let total_score: usize = rounds.iter().map(|r| r.our_score()).sum();
    dbg!(total_score);




    // println!(
    //     "{round:?}: outcome={outcome:?}, our score={out_score}",
    //     outcome = round.outcome(),
    //     out_score = round.our_score(),
    // );

    Ok(())
}

// // Win conditions [R < P < S]

// enum Object {
//     Figure(char),
// }

// fn get_index(fig: Object) -> i32 {
//     match fig {
//         Object::Figure(c) => match c {
//             'A' | 'X' => 1,
//             'B' | 'Y' => 2,
//             'C' | 'Z' => 3,
//             _ => -1,
//         },
//     }
// }

// fn main() {
//     let file = std::fs::read_to_string("input.txt").unwrap();

//     let mut opponnent_index: i32 = -1;
//     let mut me_index: i32 = -1;
//     let mut score = 0;
//     for line in file.lines() {
//         let vars = line.split_whitespace().take(2).collect::<Vec<&str>>();
//         if let [oponnent, me] = vars[..] {
//             opponnent_index = get_index(Object::Figure(
//                 oponnent.chars().take(1).collect::<Vec<char>>()[0],
//             ));
//             me_index = get_index(Object::Figure(me.chars().take(1).collect::<Vec<char>>()[0]));
//         }

//         if me_index == 2 {
//             me_index = opponnent_index;
//         } else if me_index == 1 {
//             if opponnent_index == 1 {
//                 me_index = 3;
//             } else if opponnent_index == 3 {
//                 me_index = 2;
//             } else if opponnent_index == 2 {
//                 me_index = 1
//             }
//         } else if me_index == 3 {
//             if opponnent_index == 1 {
//                 me_index = 2;
//             } else if opponnent_index == 3 {
//                 me_index = 1;
//             } else if opponnent_index == 2 {
//                 me_index = 3
//             }
//         }

//         score += me_index;
//         if opponnent_index == 3 && me_index == 1 {
//             score += 6;
//         } else if opponnent_index == 1 && me_index == 3 {
//             score += 0;
//         } else if opponnent_index > me_index {
//             // Nothing
//         } else if opponnent_index < me_index {
//             score += 6;
//         } else if opponnent_index == me_index {
//             score += 3;
//         }
//     }

//     println!("Score: {}", score);
// }

