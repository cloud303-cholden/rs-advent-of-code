use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let assignments: Vec<_> = input
        // Split on newlines to get all pairs.
        .lines()
        .map(|pair| {
            // For every pair, split on commas to get the sections.
            pair.split(',')
                .map(|sections| {
                    // For every section range, split on hyphens to get the lower and upper bounds
                    // of the section range.
                    sections
                        .split('-')
                        .map(|section| {
                            // For every section bound, parse to `u32`.
                            section
                                .parse::<u32>()
                                .unwrap()
                        })
                        // Collect the section bounds to a tuple. This simplifies the `filter`
                        // expression for constructing `overlaps` below.
                        // Very useful!
                        .collect_tuple::<(u32, u32)>()
                        .unwrap()
                })
                // Collect the section ranges to a tuple. This simplifies the `filter`
                // expression for constructing `overlaps` below.
                .collect_tuple()
                .unwrap()
        })
        // Collect to the `assignments` vector.
        .collect();

    #[rustfmt::skip]
    let overlaps: u32 = assignments
        .iter()
        // Filter for pairs where one the section ranges fully contains the other.
        .filter(|(a, b)| {
            ((a.0 <= b.0) && (a.1 >= b.1))
            || ((b.0 <= a.0) && (b.1 >= a.1))
        })
        // Count the number of instances and try into `u32`.
        .count()
        .try_into()
        .unwrap();

    Some(overlaps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let assignments: Vec<_> = input
        // Split on newlines to get all pairs.
        .lines()
        .map(|pair| {
            // For every pair, split on commas to get the sections.
            pair.split(',')
                .map(|sections| {
                    // For every section range, split on hyphens to get the lower and upper bounds
                    // of the section range.
                    sections
                        .split('-')
                        .map(|section| {
                            // For every section bound, parse to `u32`.
                            section
                                .parse::<u32>()
                                .unwrap()
                        })
                        // Collect the section bounds to a tuple. This simplifies the `filter`
                        // expression for constructing `overlaps` below.
                        // Very useful!
                        .collect_tuple::<(u32, u32)>()
                        .unwrap()
                })
                // Collect the section ranges to a tuple. This simplifies the `filter`
                // expression for constructing `overlaps` below.
                .collect_tuple()
                .unwrap()
        })
        // Collect to the `assignments` vector.
        .collect();

    #[rustfmt::skip]
    let overlaps: u32 = assignments
        .iter()
        // Filter for pairs where one the section ranges partially contains the other.
        .filter(|(a, b)| {
            ((a.0 <= b.0) && (a.1 >= b.0))
            || ((b.0 <= a.0) && (b.1 >= a.0))
        })
        // Count the number of instances and try into `u32`.
        .count()
        .try_into()
        .unwrap();

    Some(overlaps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
