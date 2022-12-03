use strum::*;

#[derive(EnumString)]
pub enum OpponentChoice {
    A,
    B,
    C,
}

impl OpponentChoice {
    pub fn to_choice(self) -> Choice {
        use Choice::*;
        use OpponentChoice::*;
        match self {
            A => Rock,
            B => Paper,
            C => Scissors,
        }
    }
}

#[derive(EnumString)]
pub enum MyChoice {
    X,
    Y,
    Z,
}

impl MyChoice {
    pub fn to_choice(self) -> Choice {
        use Choice::*;
        use MyChoice::*;
        match self {
            X => Rock,
            Y => Paper,
            Z => Scissors,
        }
    }
}

#[derive(EnumString)]
pub enum DesiredOutcome {
    X,
    Y,
    Z,
}

impl DesiredOutcome {
    pub fn to_outcome(self) -> Outcome {
        use DesiredOutcome::*;
        use Outcome::*;

        match self {
            X => Lose,
            Y => Draw,
            Z => Win,
        }
    }
}

pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn value(self) -> u32 {
        use Choice::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    pub fn opponent(opponent_choice: &OpponentChoice) -> Self {
        use Choice::*;
        use OpponentChoice::*;
        match opponent_choice {
            A => Rock,
            B => Paper,
            C => Scissors,
        }
    }

    pub fn to_opponent(self) -> OpponentChoice {
        use Choice::*;
        use OpponentChoice::*;
        match self {
            Rock => A,
            Paper => B,
            Scissors => C,
        }
    }

    pub fn my(my_choice: &MyChoice) -> Self {
        use Choice::*;
        use MyChoice::*;
        match my_choice {
            X => Rock,
            Y => Paper,
            Z => Scissors,
        }
    }

    pub fn to_my(self) -> MyChoice {
        use Choice::*;
        use MyChoice::*;
        match self {
            Rock => X,
            Paper => Y,
            Scissors => Z,
        }
    }

    pub fn from_outcome(outcome: &Outcome, choice: &Choice) -> Self {
        use Choice::*;
        use Outcome::*;

        // More questionable decisions...
        match outcome {
            Win => match choice {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            Draw => match choice {
                Rock => Rock,
                Paper => Paper,
                Scissors => Scissors,
            },
            Lose => match choice {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
        }
    }
}

pub enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn value(self) -> u32 {
        use Outcome::*;
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }

    pub fn new(my_choice: &Choice, opponent_choice: &Choice) -> Self {
        use Choice::*;
        use Outcome::*;

        // Definitely a naive approach, but it gets the job done.
        match (my_choice, opponent_choice) {
            (Rock, Paper) => Lose,
            (Paper, Rock) => Win,
            (Scissors, Rock) => Lose,
            (Rock, Scissors) => Win,
            (Scissors, Paper) => Win,
            (Paper, Scissors) => Lose,
            (Rock, Rock) => Draw,
            (Scissors, Scissors) => Draw,
            (Paper, Paper) => Draw,
        }
    }

    pub fn desired(desired_outcome: &DesiredOutcome) -> Self {
        use DesiredOutcome::*;
        use Outcome::*;
        match desired_outcome {
            X => Lose,
            Y => Draw,
            Z => Win,
        }
    }
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or_else(|| input.strip_suffix('\n'))
        .unwrap_or(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    // Strip trailing newline to avoid panicking at
    // 'called `Result::unwrap()` on an `Err` value: VariantNotFound'.
    // Would be better to handle the error rather than eliminate a single possible one.
    let input = strip_trailing_newline(input);

    let choices: Vec<Vec<&str>> = input
        .split('\n')
        .map(|i| i.split(' ').collect())
        .collect();

    let mut score = 0;

    for choice in choices {
        let opponent_choice: Choice = choice[0]
            .parse::<OpponentChoice>()
            .unwrap()
            .to_choice();
        let my_choice: Choice = choice[1]
            .parse::<MyChoice>()
            .unwrap()
            .to_choice();

        let outcome: Outcome = Outcome::new(&my_choice, &opponent_choice);

        score += outcome.value() + my_choice.value()
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Strip trailing newline to avoid panicking at
    // 'called `Result::unwrap()` on an `Err` value: VariantNotFound'.
    // Would be better to handle the error rather than eliminate a single possible one.
    let input = strip_trailing_newline(input);

    let strategies: Vec<Vec<&str>> = input
        .split('\n')
        .map(|i| i.split(' ').collect())
        .collect();

    let mut score = 0;

    for strategy in strategies {
        let opponent_choice: Choice = strategy[0]
            .parse::<OpponentChoice>()
            .unwrap()
            .to_choice();
        let desired_outcome: Outcome = strategy[1]
            .parse::<DesiredOutcome>()
            .unwrap()
            .to_outcome();

        let my_choice: Choice = Choice::from_outcome(&desired_outcome, &opponent_choice);
        let outcome: Outcome = Outcome::new(&my_choice, &opponent_choice);

        score += outcome.value() + my_choice.value()
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);

    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
