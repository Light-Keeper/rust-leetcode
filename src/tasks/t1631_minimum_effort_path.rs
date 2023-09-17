// https://leetcode.com/problems/path-with-minimum-effort/

use std::collections::{BinaryHeap};


impl Solution {
    pub fn minimum_effort_path(mut heights: Vec<Vec<i32>>) -> i32 {
        let n = heights.len();
        let m = heights[0].len();
        let mut ans = 0;

        let mut queue = BinaryHeap::<(i32, u16)>::new();
        queue.push((0, (((n-1) << 8) | (m - 1))  as u16));

        loop {
            let (mut x, combined_index) = queue.pop().unwrap();
            x = -x;

            let i = (combined_index >> 8) as usize;
            let j = (combined_index & 0xFF) as usize;

            if heights[i][j] < 0 {
                continue
            }

            if x > ans {
                ans = x
            }

            if i == 0 && j == 0 {
                return ans
            }

            let mut add = |a: usize, b: usize| {
                if heights[a][b] < 0 {
                    return
                }

                let diff = (heights[a][b] - heights[i][j]).abs();
                queue.push((-diff, ((a << 8) | b) as u16));
            };

            if i > 0 { add(i - 1, j) }
            if j > 0 { add(i, j - 1) }
            if i < n-1 { add(i + 1, j) }
            if j < m-1 { add(i, j + 1) }

            heights[i][j] = -1;
        }
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::minimum_effort_path(
            vec![
                vec![1,2,2],
                vec![3,8,2],
                vec![5,3,5]
            ]));

        assert_eq!(0, Solution::minimum_effort_path(
            vec![
                vec![1,2,1,1,1],
                vec![1,2,1,2,1],
                vec![1,2,1,2,1],
                vec![1,2,1,2,1],
                vec![1,1,1,2,1]
            ]
        ));

        assert_eq!(9, Solution::minimum_effort_path(vec![vec![1,10,6,7,9,10,4,9]]));

        assert_eq!(5, Solution::minimum_effort_path(
            vec![
               vec![ 4, 3, 4,10, 5, 5, 9, 2],
               vec![10, 8, 2,10, 9, 7, 5, 6],
               vec![ 5, 8,10,10,10, 7, 4, 2],
               vec![ 5, 1, 3, 1, 1, 3, 1, 9],
               vec![ 6, 4,10, 6,10, 9, 4, 6]
           ]))
    }
}