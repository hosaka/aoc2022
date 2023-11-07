use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {value:?}")),
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (Some(theirs), Some(' '), Some(outcome), None) =
            (chars.next(), chars.next(), chars.next(), chars.next())
        else {
            return Err(color_eyre::eyre::eyre!(
                "expected <theirs>SP<outcome>EOL, got {s:?}"
            ));
        };

        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome::try_from(outcome)?;
        let ours = outcome.matching_move(theirs);

        Ok(Self { theirs, ours })
    }
}

#[derive(Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        return match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {value:?}")),
        };
    }
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn points(self) -> usize {
        return match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };
    }

    fn beats(self, other: Move) -> bool {
        return matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        );
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            return Outcome::Win;
        } else if theirs.beats(self) {
            return Outcome::Loss;
        } else {
            return Outcome::Draw;
        }
    }

    fn winning_move(self) -> Self {
        return Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move beats us");
    }

    fn losing_move(self) -> Self {
        return Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("we beat at least one move");
    }

    fn drawing_move(self) -> Self {
        return self;
    }
}

impl Outcome {
    fn points(self) -> usize {
        return match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        };
    }

    fn matching_move(self, theirs: Move) -> Move {
        return match self {
            Outcome::Win => theirs.winning_move(),
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Loss => theirs.losing_move(),
        };
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        return self.ours.outcome(self.theirs);
    }
    fn score(self) -> usize {
        return self.ours.points() + self.outcome().points();
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // for round in include_str!("../input.txt")
    //     .lines()
    //     .map(|line| line.parse::<Round>())
    // {
    //     let round = round?;
    //     println!(
    //         "{round:?}: outcome={outcome:?}, score={score}",
    //         outcome = round.outcome(),
    //         score = round.score()
    //     );
    // }

    let rounds: Vec<Round> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<Round>())
        .collect::<Result<_, _>>()?;
    let score: usize = rounds.iter().map(|round| round.score()).sum();

    dbg!(score);
    Ok(())
}
