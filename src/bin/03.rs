use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let items: Vec<(HashSet<char>, HashSet<char>)> = input
        .lines()
        .map(|i| i.split_at(i.len() / 2))
        .map(|(a, b)| {
            (
                a.to_string()
                    .chars()
                    .collect(),
                b.to_string()
                    .chars()
                    .collect(),
            )
        })
        .collect();

    let alphabet: HashMap<char, u32> = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .enumerate()
        .map(|(i, c)| (c, (i as u32) + 1))
        .collect();

    let mut total = 0;
    for rucksack in items {
        let duplicate = rucksack
            .0
            .intersection(&rucksack.1)
            .next();
        let duplicate = match duplicate {
            Some(duplicate) => duplicate,
            _ => continue,
        };

        let dup_value = alphabet.get(duplicate);
        match dup_value {
            Some(dup_value) => total += dup_value,
            _ => continue,
        }
    }

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
