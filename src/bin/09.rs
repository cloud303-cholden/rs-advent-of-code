use std::collections::HashSet;
use strum::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[rustfmt::skip]
impl Point {
    pub fn is_touching(&self, other: &Point) -> bool{
        if (self.x - other.x).abs() < 2 && (self.y - other.y).abs() < 2 {
            return true
        }
        false
    }

    pub fn compute_tail_motion(&self, tail_point: Point) -> Vec<Motion> {
        use Direction::*;
        let x_distance = self.x - tail_point.x;
        let y_distance = self.y - tail_point.y;

        match (x_distance, y_distance) {
            (0, 2) => vec![Motion { direction: U, distance: 1 }],
            (2, 0) => vec![Motion { direction: R, distance: 1 }],
            (1, 2) => vec![Motion { direction: R, distance: 1 }, Motion { direction: U, distance: 1 }],
            (2, 1) => vec![Motion { direction: R, distance: 1 }, Motion { direction: U, distance: 1 }],
            (2, 2) => vec![Motion { direction: R, distance: 1 }, Motion { direction: U, distance: 1 }],
            (0, -2) => vec![Motion { direction: D, distance: 1 }],
            (-2, 0) => vec![Motion { direction: L, distance: 1 }],
            (-1, -2) => vec![Motion { direction: L, distance: 1 }, Motion { direction: D, distance: 1 }],
            (-2, -2) => vec![Motion { direction: L, distance: 1 }, Motion { direction: D, distance: 1 }],
            (-1, 2) => vec![Motion { direction: L, distance: 1 }, Motion { direction: U, distance: 1 }],
            (-2, 2) => vec![Motion { direction: L, distance: 1 }, Motion { direction: U, distance: 1 }],
            (1, -2) => vec![Motion { direction: R, distance: 1 }, Motion { direction: D, distance: 1 }],
            (-2, -1) => vec![Motion { direction: L, distance: 1 }, Motion { direction: D, distance: 1 }],
            (-2, 1) => vec![Motion { direction: L, distance: 1 }, Motion { direction: U, distance: 1 }],
            (2, -1) => vec![Motion { direction: R, distance: 1 }, Motion { direction: D, distance: 1 }],
            (2, -2) => vec![Motion { direction: R, distance: 1 }, Motion { direction: D, distance: 1 }],
            _ => vec![Motion { direction: U, distance: 0 }],
        }
    }
}

#[derive(EnumString, Debug)]
pub enum Direction {
    R,
    L,
    U,
    D,
}

#[derive(Debug)]
pub struct Motion {
    pub direction: Direction,
    pub distance: u32,
}

impl Motion {
    pub fn new(motion: &str) -> Self {
        let mut motion = motion.split_whitespace();
        let direction = motion
            .next()
            .unwrap()
            .parse::<Direction>()
            .unwrap();
        let distance = motion
            .next()
            .unwrap()
            .parse()
            .unwrap();
        Self {
            direction,
            distance,
        }
    }

    pub fn create_point(&self, point: Point) -> Point {
        match self.direction {
            Direction::R => Point {
                x: point.x + 1,
                y: point.y,
            },
            Direction::L => Point {
                x: point.x - 1,
                y: point.y,
            },
            Direction::U => Point {
                x: point.x,
                y: point.y + 1,
            },
            Direction::D => Point {
                x: point.x,
                y: point.y - 1,
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let motions: Vec<Motion> = input
        .lines()
        .map(Motion::new)
        .collect();

    let mut head_point = Point { x: 0, y: 0 };
    let mut tail_point = head_point;
    let mut unique_points: HashSet<Point> = HashSet::new();
    unique_points.insert(tail_point);

    for motion in motions.iter() {
        for _step in 0..motion.distance {
            head_point = motion.create_point(head_point);
            if !&tail_point.is_touching(&head_point) {
                let tail_motions = &head_point.compute_tail_motion(tail_point);
                for tail_motion in tail_motions.iter() {
                    if tail_motion.distance == 0 {
                        panic!("Tail falling behind!");
                    }
                    tail_point = tail_motion.create_point(tail_point);
                }
                unique_points.insert(tail_point);
            }
        }
    }

    Some(unique_points.len())
}

pub trait Rope {
    fn update_point(&mut self, idx: usize);
}

impl Rope for Vec<Point> {
    fn update_point(&mut self, idx: usize) {
        let mut point = self[idx];
        let parent_point = self[idx - 1];
        println!("Check:  {:?} ({})", point, idx);

        if !point.is_touching(&parent_point) {
            let motions = &parent_point.compute_tail_motion(point);
            for motion in motions.iter() {
                if motion.distance == 0 {
                    panic!("Tail falling behind!");
                }
                point = motion.create_point(point);
            }
            self[idx] = point;
            println!(
                "Update: {:?} ({}) with {:?} toward {:?}",
                point, idx, motions, parent_point
            );
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let motions: Vec<Motion> = input
        .lines()
        .map(Motion::new)
        .collect();

    let rope_length = 10;
    let mut rope: Vec<Point> = vec![Point { x: 0, y: 0 }; rope_length];
    let mut unique_points: HashSet<Point> = HashSet::new();
    unique_points.insert(rope[rope_length - 1]);

    for (idx, motion) in motions.iter().enumerate() {
        println!("Motion: {:?} ({})", motion, idx);
        for _step in 0..motion.distance {
            rope[0] = motion.create_point(rope[0]);
            println!("Head:   {:?}", rope[0]);
            for i in 1..rope_length {
                rope.update_point(i)
            }
            unique_points.insert(rope[rope_length - 1]);
        }
    }

    Some(unique_points.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
