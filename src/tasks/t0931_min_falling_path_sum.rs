// https://leetcode.com/problems/minimum-falling-path-sum
use super::*;

impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.len() == 1 {
            return matrix[0][0];
        }

        let n = matrix.len() - 1;

        for i in 1..=n {
            matrix[i][0] += matrix[i - 1][0].min(matrix[i - 1][1]);

            for j in 1..n {
                matrix[i][j] += matrix[i - 1][j]
                    .min(matrix[i - 1][j - 1])
                    .min(matrix[i - 1][j + 1])
            }

            matrix[i][n] += matrix[i - 1][n].min(matrix[i - 1][n - 1]);
        }

        matrix[n].iter().copied().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]),
            13
        );
    }
}
