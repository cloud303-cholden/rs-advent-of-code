use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let packet_length = input.len();

    let mut marker_idx: Option<usize> = None;
    for i in 0..packet_length {
        let marker_start = i.saturating_sub(3);
        let marker = &input[marker_start..=i];
        let uniques = marker
            .to_string()
            .chars()
            .unique()
            .collect::<Vec<_>>();
        if uniques.len() != 4 {
            continue;
        }
        marker_idx = Some(i + 1);
        break;
    }
    marker_idx
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
