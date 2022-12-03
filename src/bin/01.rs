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

pub fn part_two(input: &str) -> Option<u32> {
    let calories: Vec<Option<u32>> = input
        .split('\n')
        .map(|i| FromStr::from_str(i).ok())
        .collect();

    let mut current: u32 = 0;
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;

    // Pretty naive solution. Might come back to this.
    for calorie in calories {
        let calorie = calorie.unwrap_or(0);
        match calorie {
            0 => {
                if current > first {
                    third = second;
                    second = first;
                    first = current
                } else if current > second {
                    third = second;
                    second = current
                } else if current > third {
                    third = current
                }
                current = 0
            }
            _ => current += calorie,
        }
    }
    Some(first + second + third)
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
        assert_eq!(part_two(&input), Some(45000));
    }
}
