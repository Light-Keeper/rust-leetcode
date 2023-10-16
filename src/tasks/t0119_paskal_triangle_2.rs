// https://leetcode.com/problems/pascals-triangle-ii

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut a = Vec::with_capacity(34);
        a.push(1);

        for _ in 1..=row_index {
            a = std::iter::once(1)
                .chain(a.windows(2).map(|w| w[0] + w[1]))
                .chain(std::iter::once(1))
                .collect();
        }

        a
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {
        let ans = Solution::get_row(0);
        assert_eq!(ans, vec![1]);

        let ans = Solution::get_row(1);
        assert_eq!(ans, vec![1, 1]);

        let ans = Solution::get_row(4);
        assert_eq!(ans, vec![1, 4, 6, 4, 1]);
    }
}
