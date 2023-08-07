// https://leetcode.com/problems/two-sum/

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut i = 0;
        let mut j = n * m;

        while i + 1 < j {
            let c = (i + j) / 2;
            if target < matrix[c/m][c%m] {
                j = c
            } else {
                i = c
            }
        }

        return matrix[i/m][i%m] == target
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 3));
        assert_eq!(false, Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 12));
        assert_eq!(true, Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 1));
        assert_eq!(true, Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 60));
        assert_eq!(false, Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 61));
        assert_eq!(false, Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 0));
    }
}