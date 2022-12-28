pub enum RangeType {
    Horizontal,
    Vertical,
}

impl RangeType {
    pub fn get_2d_value<T: Copy>(&self, trees: &[Vec<T>], x: usize, y: usize) -> T {
        match self {
            RangeType::Horizontal => trees[x][y],
            RangeType::Vertical => trees[y][x],
        }
    }

    pub fn set_2d_value<T>(&self, trees: &mut [Vec<T>], x: usize, y: usize, value: T) {
        match self {
            RangeType::Horizontal => trees[x][y] = value,
            RangeType::Vertical => trees[y][x] = value,
        }
    }
}

pub fn _print_row(visibilities: &[Vec<bool>], trees: &[Vec<u32>], row: usize) {
    for (y, (vis_row, tree_row)) in visibilities
        .iter()
        .zip(trees)
        .enumerate()
    {
        for (x, (vis, tree)) in vis_row
            .iter()
            .enumerate()
            .zip(tree_row)
            .enumerate()
        {
            if y == row {
                println!("{:?}: {:?}, ({:?}, {:?})", tree, vis.1, x, y);
            }
        }
    }
}

pub fn check_visibility<T, U>(
    outer_range: T,
    inner_range: U,
    trees: &[Vec<u32>],
    visibilities: &mut [Vec<bool>],
    outer_range_type: RangeType,
) where
    T: std::clone::Clone + std::iter::ExactSizeIterator<Item = usize>,
    U: std::clone::Clone + std::iter::ExactSizeIterator<Item = usize>,
{
    let mut tallest: u32;
    let outer_end = outer_range.len();
    let inner_end = inner_range.len();

    for y in outer_range {
        tallest = 0;
        for x in inner_range.clone() {
            let tree = outer_range_type.get_2d_value(trees, x, y);
            if x == 0 || y == 0 || x == inner_end - 1 || y == outer_end - 1 {
                outer_range_type.set_2d_value(visibilities, x, y, true);
            }
            if tree > tallest {
                outer_range_type.set_2d_value(visibilities, x, y, true);
                tallest = tree;
            }
            if tallest == 9 {
                break;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let trees = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let mut visibilities = trees
        .iter()
        .map(|l| {
            l.iter()
                .map(|_| false)
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<_>>();

    let height = trees.len();
    let width = trees.first()?.len();

    // Check from left to right, going down the rows of trees
    check_visibility(
        0..height,
        0..width,
        &trees,
        &mut visibilities,
        RangeType::Vertical,
    );
    // Check from right to left, going down the rows of trees
    check_visibility(
        0..height,
        (0..width).rev(),
        &trees,
        &mut visibilities,
        RangeType::Vertical,
    );
    // Check from top to bottom, going across the columns of trees
    check_visibility(
        0..width,
        0..height,
        &trees,
        &mut visibilities,
        RangeType::Horizontal,
    );
    // Check from bottom to top, going across the columns of trees
    check_visibility(
        0..width,
        (0..height).rev(),
        &trees,
        &mut visibilities,
        RangeType::Horizontal,
    );

    let number_of_trees: usize = visibilities
        .iter()
        .map(|l| {
            l.iter()
                .filter(|c| **c)
                .count()
        })
        .sum();

    Some(number_of_trees)
}

pub fn check_vertical_score<T>(range: T, x: usize, trees: &[Vec<u32>]) -> usize
where
    T: std::clone::Clone + std::iter::Iterator<Item = usize>,
{
    let first_idx = range.clone().next();
    if first_idx.is_none() {
        return 0;
    }

    let first = trees[first_idx.unwrap()][x];
    for (count, y) in range
        .clone()
        .skip(1)
        .enumerate()
    {
        let tree = trees[y][x];
        if tree >= first {
            return count + 1;
        }
    }
    range.count() - 1
}

pub fn check_horizontal_score<T>(range: T, y: usize, trees: &[Vec<u32>]) -> usize
where
    T: std::clone::Clone + std::iter::Iterator<Item = usize>,
{
    let first_idx = range.clone().next();
    if first_idx.is_none() {
        return 0;
    }

    let first = trees[y][first_idx.unwrap()];
    for (count, x) in range
        .clone()
        .skip(1)
        .enumerate()
    {
        let tree = trees[y][x];
        if tree >= first {
            return count + 1;
        }
    }
    range.count() - 1
}

pub fn part_two(input: &str) -> Option<usize> {
    let trees = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let height = trees.len() - 1;
    let width = trees.first()?.len() - 1;
    let mut highest_score: usize = 0;

    for (y, tree_row) in trees.iter().enumerate() {
        for (x, _tree) in tree_row.iter().enumerate() {
            let top = check_vertical_score((0..=y).rev(), x, &trees);
            let bottom = check_vertical_score(y..=height, x, &trees);
            let left = check_horizontal_score((0..=x).rev(), y, &trees);
            let right = check_horizontal_score(x..=width, y, &trees);

            let score = top * bottom * left * right;
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    Some(highest_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
