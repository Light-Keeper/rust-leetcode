// https://leetcode.com/problems/monotonic-array/

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut x = 3;
        
        for i in 1..nums.len() {
            if nums[i-1] < nums[i] { x &= 1 }
            if nums[i] < nums[i-1] { x &= 2 }
        }
        
        x != 0
    }
}


use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_monotonic(vec![1,2,2,3]));
        assert!(Solution::is_monotonic(vec![3]));
        assert!(Solution::is_monotonic(vec![3, 3, 2, 1]));
        assert!(!Solution::is_monotonic(vec![1, 3, 3, 2, 1]));
    }
}
