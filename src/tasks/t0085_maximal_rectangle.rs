// https://leetcode.com/problems/maximal-rectangle/

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut maximums = vec![0; matrix[0].len()];
        let mut ans = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == '0' {
                    maximums[j] = i+1
                }
            }

            for j in 0..matrix[i].len() {
                let mut sliding_max = maximums[j];
                for t in j..matrix[i].len() {
                    sliding_max = sliding_max.max(maximums[t]);
                    if sliding_max == i + 1 {
                        break
                    }
                    let candidate = (i + 1 - sliding_max) * (t + 1 - j);
                    ans = ans.max(candidate)
                }
            }
        }

        return ans as i32
    }
}
use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, Solution::maximal_rectangle(
            vec![
                vec!['1','0','1','0','0'],
                vec!['1','0','1','1','1'],
                vec!['1','1','1','1','1'],
                vec!['1','0','0','1','0']
            ]
        ));

        assert_eq!(0, Solution::maximal_rectangle(
            vec![
                vec!['0'],
            ]
        ));

        assert_eq!(1, Solution::maximal_rectangle(
            vec![
                vec!['1'],
            ]
        ));
    }
}