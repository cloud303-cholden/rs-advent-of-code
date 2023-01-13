use itertools::Itertools;
use std::collections::VecDeque;

pub const NEIGHBORS: [(i32, i32); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

#[derive(Copy, Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

pub type ScoredNodes = VecDeque<Node>;
pub type Nodes = Vec<Node>;
pub type Map = Vec<Nodes>;

#[derive(Copy, Clone)]
pub struct Node {
    f: u32,
    h: u32,
    g: u32,
    height: u8,
    position: Position,
    parent: Option<Position>,
    closed: bool,
}

impl Node {
    pub fn new(height: u8, position: Position) -> Self {
        Self {
            height,
            position,
            ..Default::default()
        }
    }

    pub fn g_cost(&mut self, other: &Self) {
        self.g = (self
            .position
            .x
            .abs_diff(other.position.x)
            + self
                .position
                .y
                .abs_diff(other.position.y)) as u32;
    }

    pub fn h_cost(&mut self, other: &Self) {
        self.h = (self
            .position
            .x
            .abs_diff(other.position.x)
            + self
                .position
                .y
                .abs_diff(other.position.y)) as u32;
    }

    pub fn f_cost(&mut self) {
        self.f = self.h + self.g;
    }

    pub fn update_costs(&mut self, target: &Self, source: &Self) {
        self.g_cost(source);
        self.h_cost(target);
        self.f_cost();
    }

    pub fn find_insert_idx(&self, nodes: &ScoredNodes) -> usize {
        nodes.partition_point(|&n| (n.f < self.f) || (n.f == self.f && n.h < self.h))
    }

    pub fn find_neighbors(&self, target: &Self, source: &Self, node_map: &mut Map) -> Nodes {
        let mut neighbor_nodes = Vec::with_capacity(8);

        for (x, y) in NEIGHBORS {
            let x_pos = self.position.x as i32 + x;
            let y_pos = self.position.y as i32 + y;
            if x_pos < 0 || y_pos < 0 {
                continue;
            }

            node_map[x_pos as usize][y_pos as usize].update_costs(target, source);
            let node = node_map[x_pos as usize][y_pos as usize];
            if node.closed {
                continue;
            }
            if (97..=node.height + 1).contains(&self.height) {
                neighbor_nodes.push(node);
            }
        }

        neighbor_nodes
    }

    pub fn create_path(&self, target_node: Node, map: &Map) -> Nodes {
        let mut current_node = target_node;
        let mut path: Nodes = vec![current_node];

        while let Some(parent_pos) = current_node.parent {
            let parent_node = map[parent_pos.x][parent_pos.y];
            path.push(parent_node);
            current_node = parent_node;
        }

        path
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            f: 0,
            g: 0,
            h: 0,
            height: 97,
            position: Position { x: 0, y: 0 },
            parent: None,
            closed: false,
        }
    }
}

pub fn astar(start_node: Node, target_node: Node, input: &str) -> Option<usize> {
    let mut node_map = str_to_map(input);
    let length = node_map
        .iter()
        .flatten()
        .count();
    let mut open_nodes: ScoredNodes = VecDeque::with_capacity(length);
    // let mut closed_nodes: Nodes = Vec::with_capacity(length);

    open_nodes.push_front(start_node);

    let mut current_node = open_nodes[0];
    let target_idx: Option<usize> = None;
    while target_idx.is_none() {
        let _neighbors = current_node.find_neighbors(&target_node, &start_node, &mut node_map);
        current_node.closed = true;
        if let Some(n) = open_nodes.pop_front() {
            current_node = n;
        } else {
            return None;
        }
    }
    None
}

pub fn str_to_map(input: &str) -> Vec<Nodes> {
    let width = &input
        .lines()
        .next()
        .unwrap()
        .len();
    let height = &input.lines().count();
    let node_map: Vec<Vec<u8>> = input
        .as_bytes()
        .iter()
        .filter(|b| **b != 10)
        .enumerate()
        .sorted_by(|(a, _), (b, _)| Ord::cmp(&(a % width), &(b % width)))
        .map(|(_, b)| *b)
        .into_iter()
        .chunks(*height)
        .into_iter()
        .map(|c| c.collect())
        .collect();

    node_map
        .iter()
        .enumerate()
        .map(|(x, r)| {
            r.iter()
                .enumerate()
                .map(|(y, b)| Node::new(*b, Position { x, y }))
                .collect()
        })
        .collect()
}

pub fn part_one(_input: &str) -> Option<u32> {
    let _heights: [u8; 26] = core::array::from_fn(|i| i as u8 + 97);
    let _current = 83u8;
    let _signal = 69u8;

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
