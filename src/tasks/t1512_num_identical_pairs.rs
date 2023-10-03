// https://leetcode.com/problems/number-of-good-pairs/

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counters = [0u8; 101];
        nums.into_iter().for_each(|x| { counters[x as usize] += 1 });
        
        counters.into_iter()
            .map(|x| x as i32)
            .map(|x| x * (x - 1) / 2)
            .sum()
    }
}


use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_identical_pairs(vec![1,2,3,1,1,3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1,2,3]), 0);
        assert_eq!(Solution::num_identical_pairs(vec![1,1,1,1]), 6);
    }
}
