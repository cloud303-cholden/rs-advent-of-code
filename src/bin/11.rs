#![feature(int_roundings)]

use std::collections::VecDeque;
use std::iter::IntoIterator;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Monkey<'a> {
    items: VecDeque<u32>,
    operation: (&'a str, &'a str),
    test: u32,
    test_true: usize,
    test_false: usize,
}

impl<'a> FromIterator<&'a str> for Monkey<'a> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut lines = iter.into_iter();
        lines.next();

        // I hate this. But first time implementing `FromIterator`, so I'll keep it!
        Self {
            items: lines
                .next()
                .unwrap()
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .into_iter()
                .map(|c| c.parse().unwrap())
                .collect::<VecDeque<u32>>(),
            operation: lines
                .next()
                .unwrap()
                .strip_prefix("  Operation: new = old ")
                .unwrap()
                .split_once(' ')
                .unwrap(),
            test: lines
                .next()
                .unwrap()
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap(),
            test_true: lines
                .next()
                .unwrap()
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
            test_false: lines
                .next()
                .unwrap()
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rounds = 20;

    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .into_iter()
        .map(|t| Monkey::from_iter(t.lines()))
        .collect();
    println!("{monkeys:?}");

    let mut queue: Vec<VecDeque<u32>> = vec![VecDeque::new(); monkeys.len()];

    let mut inspections = vec![0u32; monkeys.len()];

    (0..rounds).for_each(|round| {
        monkeys
            .iter_mut()
            .enumerate()
            .for_each(|(i, monkey)| {
                println!("\nMonkey {i}, Round {round}");
                println!("Found items in queue: {:?}", queue[i]);
                monkey
                    .items
                    .append(&mut queue[i]);
                println!(
                    "Moved items in queue to monkey: {:?}, {queue:?}",
                    monkey.items
                );
                while !monkey.items.is_empty() {
                    let mut item = monkey
                        .items
                        .pop_front()
                        .unwrap();

                    inspections[i] += 1;

                    match monkey.operation.0 {
                        "+" if monkey.operation.1 == "old" => item *= 2,
                        "+" => {
                            item += monkey
                                .operation
                                .1
                                .parse::<u32>()
                                .unwrap()
                        }
                        "*" if monkey.operation.1 == "old" => item *= item,
                        "*" => {
                            item *= monkey
                                .operation
                                .1
                                .parse::<u32>()
                                .unwrap()
                        }
                        _ => (),
                    }

                    item = item.div_floor(3);

                    match item % monkey.test == 0 {
                        true => queue[monkey.test_true].push_back(item),
                        false => queue[monkey.test_false].push_back(item),
                    }
                    println!("Added items to queue: {queue:?}");
                }
            });
        queue
            .iter_mut()
            .zip(monkeys.iter_mut())
            .for_each(|(q, m)| m.items.append(q));
        println!("{round}, {monkeys:?}");
    });

    inspections.sort();

    let monkey_business: u32 = inspections
        .iter()
        .rev()
        .take(2)
        .product();

    Some(monkey_business)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
