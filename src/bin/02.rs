use strum::*;

#[derive(EnumString)]
pub enum OpponentChoice {
    A,
    B,
    C,
}

#[derive(EnumString)]
pub enum MyChoice {
    X,
    Y,
    Z,
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

    pub fn my(my_choice: &MyChoice) -> Self {
        use Choice::*;
        use MyChoice::*;
        match my_choice {
            X => Rock,
            Y => Paper,
            Z => Scissors,
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

    pub fn new(my_choice: &MyChoice, opponent_choice: &OpponentChoice) -> Self {
        use Choice::*;
        use Outcome::*;
        let my_choice = Choice::my(my_choice);
        let opponent_choice = Choice::opponent(opponent_choice);

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
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or_else(|| input.strip_suffix('\n'))
        .unwrap_or(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    // Strip trailing newline to avoid panicking at
    // 'called `Result::unwrap()` on an `Err` value: VariantNotFound'
    let input = strip_trailing_newline(input);

    let choices: Vec<Vec<&str>> = input.split('\n').map(|i| i.split(' ').collect()).collect();

    let mut score = 0;

    for choice in choices {
        let opponent_choice: OpponentChoice = choice[0].parse().unwrap();
        let my_choice: MyChoice = choice[1].parse().unwrap();

        let outcome: Outcome = Outcome::new(&my_choice, &opponent_choice);

        score += outcome.value() + Choice::my(&my_choice).value()
    }

    Some(score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
