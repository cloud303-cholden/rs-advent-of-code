use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let mut fs: HashMap<String, u64> = HashMap::new();
    let root_dir = String::from("/");
    let mut current_dir: String = root_dir.clone();
    fs.insert(root_dir.clone(), 0);
    for line in input.lines() {
        let mut elements = line.split_whitespace();
        match elements.next() {
            Some("$") => match elements.next() {
                Some("cd") => match elements.next() {
                    Some("..") => {
                        let mut dirs = current_dir
                            .split('/')
                            .collect::<Vec<&str>>();
                        dirs.remove(dirs.len().saturating_sub(2));
                        current_dir = dirs.join("/");
                    }
                    Some("/") => {
                        current_dir = root_dir.clone();
                    }
                    Some(dir) => {
                        let dir: String = current_dir + dir + "/";
                        current_dir = dir.to_string();
                        fs.entry(dir.to_string())
                            .or_insert(0);
                    }
                    _ => continue,
                },
                _ => continue,
            },
            Some("dir") => {
                let mut dir = String::from(elements.next().unwrap());
                dir = current_dir.clone() + &dir + "/";
                fs.insert(dir, 0);
            }
            Some(size) => {
                let size: u64 = size.parse().unwrap();
                fs.entry(current_dir.clone())
                    .and_modify(|s| *s += size)
                    .or_insert(0);

                let parents = current_dir
                    .matches('/')
                    .count()
                    - 1;
                let mut dirs = current_dir
                    .split('/')
                    .collect::<Vec<&str>>();
                for _ in 1..=parents {
                    dirs.remove(dirs.len().saturating_sub(2));
                    let parent_dir = dirs.join("/");
                    fs.entry(parent_dir)
                        .and_modify(|s| *s += size)
                        .or_insert(0);
                }
            }
            _ => continue,
        }
    }

    fs.retain(|_, v| v <= &mut 100000);

    let total: u64 = fs.values().sum();
    Some(total)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
