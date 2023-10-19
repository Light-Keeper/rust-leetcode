// https://leetcode.com/problems/surrounded-regions/

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        let m = board[0].len();

        use std::iter;
        let starting_points = iter::empty()
            .chain((0..m).map(|i| (0, i)))
            .chain((0..m).map(|i| (n - 1, i)))
            .chain((0..n).flat_map(|i| iter::once((i, 0)).chain(iter::once((i, m - 1)))));

        for (i, j) in starting_points {
            Solution::dfs(board, i, j);
        }

        for row in board {
            for el in row {
                if *el == 'O' {
                    *el = 'X'
                }
                if *el == 'T' {
                    *el = 'O'
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if board[i][j] != 'O' {
            return;
        }

        board[i][j] = 'T';

        if i + 1 < board.len() {
            Solution::dfs(board, i + 1, j)
        }

        if i > 0 {
            Solution::dfs(board, i - 1, j)
        }

        if j + 1 < board[0].len() {
            Solution::dfs(board, i, j + 1)
        }

        if j > 0 {
            Solution::dfs(board, i, j - 1)
        }
    }
}

use super::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];

        Solution::solve(&mut board);

        let ans = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];

        assert_eq!(board, ans);
    }
}
