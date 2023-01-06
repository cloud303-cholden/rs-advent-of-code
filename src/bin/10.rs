pub fn part_one(input: &str) -> Option<i32> {
    let mut instructions: Vec<(i32, i32)> = input
        .lines()
        .flat_map(|l| match l {
            "noop" => vec![0_i32],
            _ => vec![
                0,
                l.split_once(' ')
                    .unwrap()
                    .1
                    .parse::<i32>()
                    .unwrap(),
            ],
        })
        .map(|e| (e, 0_i32))
        .collect();

    let mut total_signal: i32 = 1;
    for i in instructions.iter_mut() {
        total_signal += i.0;
        i.1 = total_signal
    }

    // For some reason, my interpreted instructions are offset by +1 from the instructions that
    // yield the correct answer. My instructions look correct, but the `signal_strength` they
    // produce is incorrect. Adding an extra starting instruction fixes it though.
    instructions.insert(0, (0, 1));

    let signal_strength = instructions
        .iter()
        .enumerate()
        .filter(|&(i, _)| (i as i32 - 19).rem_euclid(40) == 0)
        .take(6)
        .map(|(i, e)| e.1 * (i as i32 + 1))
        .sum();

    Some(signal_strength)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut instructions: Vec<(i32, i32)> = input
        .lines()
        .flat_map(|l| match l {
            "noop" => vec![0_i32],
            _ => vec![
                0,
                l.split_once(' ')
                    .unwrap()
                    .1
                    .parse::<i32>()
                    .unwrap(),
            ],
        })
        .map(|e| (e, 0_i32))
        .collect();

    let mut total_signal: i32 = 1;
    for i in instructions.iter_mut() {
        total_signal += i.0;
        i.1 = total_signal
    }

    instructions.insert(0, (0, 1));
    let mut crt = String::new();
    instructions
        .iter()
        .enumerate()
        .for_each(|(cycle, (_instruction, register))| {
            match (cycle as i32)
                .rem_euclid(40)
                .saturating_sub(*register)
                .abs()
                .cmp(&1)
            {
                std::cmp::Ordering::Greater => {
                    println!(". {cycle}, {register}");
                    crt.push('.')
                }
                _ => {
                    println!("# {cycle}, {register}");
                    crt.push('#')
                }
            }
        });

    crt = crt
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && (i as i32).rem_euclid(40) == 0 {
                Some('\n')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>();
    crt.pop();

    Some(crt)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let output = "##..##..##..##..##..##..##..##..##..##..\n\
            ###...###...###...###...###...###...###.\n\
            ####....####....####....####....####....\n\
            #####.....#####.....#####.....#####.....\n\
            ######......######......######......####\n\
            #######.......#######.......#######.....\n"
            .to_string();
        let lhs = part_two(&input).unwrap();
        println!("{lhs}");
        println!("{output}");
        assert_eq!(Some(lhs), Some(output));
    }
}
