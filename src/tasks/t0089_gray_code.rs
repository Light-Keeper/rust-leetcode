// https://leetcode.com/problems/gray-code/

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..(1 << n))
            .map(|i| {
                (0..n).fold(0, |acc, bit_index| {
                    let loop_len = 1 << (bit_index + 2);
                    let val = i & (loop_len - 1);
                    let bit_val = ((loop_len >> 2) <= val && val < (loop_len >> 2) * 3) as i32;
                    acc | (bit_val << bit_index)
                })
            })
            .collect()
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
        assert_eq!(Solution::gray_code(1), vec![0, 1]);
    }
}
