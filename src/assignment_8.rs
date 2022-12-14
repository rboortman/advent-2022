use crate::{Assignment, Output};

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

fn is_visible(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    let mut visible = [true, true, true, true];
    let size = grid.len();
    let tree = grid[i][j];

    for x in 0..i {
        visible[0] = visible[0] && grid[x][j] < tree;
    }
    for x in 0..j {
        visible[1] = visible[1] && grid[i][x] < tree;
    }
    for x in j + 1..size {
        visible[2] = visible[2] && grid[i][x] < tree;
    }
    for x in i + 1..size {
        visible[3] = visible[3] && grid[x][j] < tree;
    }

    visible.into_iter().any(|b| b)
}

fn scenic_score(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    let size = grid.len();
    let mut visible_trees = [i, j, size - (j + 1), size - (i + 1)];
    let tree = grid[i][j];

    for x in (0..i).rev() {
        if grid[x][j] >= tree {
            visible_trees[0] = i - x;
            break;
        }
    }
    for x in (0..j).rev() {
        if grid[i][x] >= tree {
            visible_trees[1] = j - x;
            break;
        }
    }
    for x in j + 1..size {
        if grid[i][x] >= tree {
            visible_trees[2] = x - j;
            break;
        }
    }
    for x in i + 1..size {
        if grid[x][j] >= tree {
            visible_trees[3] = x - i;
            break;
        }
    }

    visible_trees
        .into_iter()
        .fold(1, |acc, amount| acc * (amount as u32))
}

impl Assignment for Solution {
    type Input = Vec<Vec<u32>>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        let mut result = Vec::new();
        for line in input.lines() {
            let row: Vec<u32> = line
                .chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            result.push(row);
        }
        Some(result)
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        let size = input.len();
        let mut count = 0;

        for (i, v) in input.iter().enumerate() {
            for (j, _) in v.iter().enumerate() {
                if i == 0 || j == 0 || i == size - 1 || j == size - 1 {
                    count += 1;
                    continue;
                }

                if is_visible(input, i, j) {
                    count += 1;
                }
            }
        }

        Some((count).into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let size = input.len();
        let mut max_scenic_score = 0;

        for (i, v) in input.iter().enumerate() {
            for (j, _) in v.iter().enumerate() {
                if i == 0 || j == 0 || i == size - 1 || j == size - 1 {
                    continue;
                }

                let score = scenic_score(input, i, j);
                if score > max_scenic_score {
                    max_scenic_score = score;
                }
            }
        }

        Some((max_scenic_score).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 21)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 8)
    }
}
