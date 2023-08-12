// https://leetcode.com/problems/unique-paths-ii/description/

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut current = vec![0; obstacle_grid[0].len()];
        if obstacle_grid[0][0] == 1 { return 0; }

        current[0] = 1;

        for i in 0..obstacle_grid.len() {
            if obstacle_grid[i][0] == 1 { current[0] = 0; }

            for j in 1..current.len() {
                current[j] += current[j - 1];
                if obstacle_grid[i][j] == 1 { current[j] = 0 }
            }
        }

        current.pop().unwrap()
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let i = Solution::unique_paths_with_obstacles(
            vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]
        );
        assert_eq!(i, 2)
    }
}