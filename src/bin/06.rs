use itertools::Itertools;

pub fn find_marker_idx(packet: &str, unique_characters: usize) -> Option<usize> {
    let packet_length = packet.len();

    let mut marker_idx: Option<usize> = None;
    for i in 0..packet_length {
        let marker_start = i.saturating_sub(unique_characters - 1);
        let marker = &packet[marker_start..=i];
        if marker
            .to_string()
            .chars()
            .unique()
            .count()
            != unique_characters
        {
            continue;
        }
        marker_idx = Some(i + 1);
        break;
    }
    marker_idx
}

pub fn part_one(input: &str) -> Option<usize> {
    find_marker_idx(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    find_marker_idx(input, 14)
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
        assert_eq!(part_two(&input), Some(19));
    }
}
