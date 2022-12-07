use itertools::Itertools;

// https://users.rust-lang.org/t/solved-how-to-split-string-into-multiple-sub-strings-with-given-length/10542/9
// Thank you!
pub fn sub_strings(string: String, sub_len: usize) -> Vec<String> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(string[pos..pos + len - 1].to_string());
        pos += len;
    }
    subs
}

pub fn split_input(input: &str) -> (&str, &str) {
    input
        .split_once("\n\n")
        .unwrap()
}

pub fn parse_stacks(stack_rows: &str) -> Vec<Vec<char>> {
    // Parse stack configuration into rows of items
    let mut stack_rows: Vec<Vec<String>> = stack_rows
        .lines()
        .map(|l| l.to_string())
        .map(|mut l| {
            l.push(' ');
            l
        })
        .map(|l| sub_strings(l, 4))
        .collect();

    let stack_width = stack_rows[stack_rows.len() - 1].len();
    // Remove stack numbers
    stack_rows.pop();
    stack_rows.reverse();
    // Hack to hopefully not index out of bounds
    let stack_height = stack_rows.len() * 2;

    // Invert 2D vector so that outer dimension consists in stacks rather than rows.
    let mut stacks: Vec<Vec<char>> = vec![vec![' '; stack_width]; stack_height];
    for (row_index, row) in stack_rows
        .into_iter()
        .enumerate()
    {
        for (item_index, item) in row.into_iter().enumerate() {
            let item = item.chars().nth(1).unwrap();
            if item == ' ' {
                continue;
            }
            stacks[item_index][row_index] = item;
        }
    }

    // Filter out `' '` from stacks.
    stacks
        .into_iter()
        .map(|v| {
            v.into_iter()
                .filter(|c| c != &' ')
                .collect()
        })
        .collect()
}

pub fn parse_procedures(procedures: &str) -> Vec<(usize, usize, usize)> {
    // Build procedures
    procedures
        .lines()
        .map(|l| {
            l.to_string()
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        .collect()
}

pub fn read_top_items(stacks: Vec<Vec<char>>) -> String {
    // Extract the top item from each stack into a `String`
    stacks
        .into_iter()
        .map(|stack| match stack.last() {
            Some(c) => c.to_string(),
            None => "".to_owned(),
        })
        .into_iter()
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    // Could definitely have done this one cleaner. But I'm slow enough already!
    let (stack_rows, procedures) = split_input(input);

    let mut stacks = parse_stacks(stack_rows);

    let procedures = parse_procedures(procedures);

    // Apply procedures
    for p in procedures {
        for _ in 0..p.0 {
            let item = stacks[p.1 - 1].pop().unwrap();
            stacks[p.2 - 1].push(item);
        }
    }

    let top_items = read_top_items(stacks);
    Some(top_items)
}

pub fn part_two(input: &str) -> Option<String> {
    let (stack_rows, procedures) = split_input(input);

    let mut stacks = parse_stacks(stack_rows);

    let procedures = parse_procedures(procedures);

    // Apply procedures
    for p in procedures {
        let source_stack = &mut stacks[p.1 - 1];
        let mut items = source_stack.split_off(
            source_stack
                .len()
                .saturating_sub(p.0),
        );
        stacks[p.2 - 1].append(&mut items);
    }

    let top_items = read_top_items(stacks);
    Some(top_items)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
