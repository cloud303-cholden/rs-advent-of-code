use std::cmp::Ordering;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let calories: Vec<Option<u32>> = input
        .split('\n')
        .map(|i| FromStr::from_str(i).ok())
        .collect();

    let mut current: u32 = 0;
    let mut max: u32 = 0;

    for calorie in calories {
        let calorie = calorie.unwrap_or(0);
        match calorie {
            0 => match current.cmp(&max) {
                Ordering::Greater => {
                    max = current;
                    current = 0
                }
                _ => current = 0,
            },
            _ => current += calorie,
        }
    }
    Some(max)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
