// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut scores : Vec<_> = mat
            .into_iter()
            .enumerate()
            .map(|(index, row)| { (row.into_iter().sum::<i32>(), index) })
            .collect();

        scores.sort_unstable();


        scores
            .into_iter()
            .take(k as usize)
            .map(|(_, index)| index as i32)
            .collect()
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = Solution::k_weakest_rows(
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            3,
        );

        assert_eq!(result, vec![2, 0, 3]);

        let result = Solution::k_weakest_rows(
            vec![
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
            ],
            2,
        );

        assert_eq!(result, vec![0, 2]);
    }
}
