#![feature(int_roundings)]

use itertools::Itertools;
use std::collections::VecDeque;
use std::iter::IntoIterator;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Monkey<'a> {
    items: VecDeque<u64>,
    operation: (&'a str, &'a str),
    test: u64,
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
                .collect::<VecDeque<u64>>(),
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

pub fn compute_monkey_business(rounds: u32, input: &str, reduce_worry: bool) -> Option<u64> {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .into_iter()
        .map(|t| Monkey::from_iter(t.lines()))
        .collect();
    println!("{monkeys:?}");

    // Had to look up the math for this part. Essentially boils down to using a common denominator
    // for operations that would otherwise yield numbers that are too big.
    let common_denominator: u64 = monkeys
        .iter()
        .map(|m| m.test)
        .unique()
        .product();

    let mut queue: Vec<VecDeque<u64>> = vec![VecDeque::new(); monkeys.len()];

    let mut inspections = vec![0u64; monkeys.len()];

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
                                .parse::<u64>()
                                .unwrap()
                        }
                        "*" if monkey.operation.1 == "old" => item *= item,
                        "*" => {
                            item *= monkey
                                .operation
                                .1
                                .parse::<u64>()
                                .unwrap()
                        }
                        _ => (),
                    }

                    if reduce_worry {
                        item = item.div_floor(3);
                    } else {
                        item %= common_denominator;
                    }

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

    let monkey_business: u64 = inspections
        .iter()
        .rev()
        .take(2)
        .product();

    Some(monkey_business)
}

pub fn part_one(input: &str) -> Option<u64> {
    compute_monkey_business(20, input, true)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Performance on this is terrible. Will analyze other solutions to learn from this puzzle.
    compute_monkey_business(10_000, input, false)
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
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
