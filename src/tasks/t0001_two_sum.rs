// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut visited = HashMap::new();
        visited.insert(target - nums[0], 0usize);

        for i in 1..(nums.len()) {
            if let Some(pair) = visited.get(&nums[i]) {
                return vec![*pair as i32, i as i32];
            }
            visited.insert(target - nums[i], i);
        }

        unreachable!();
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1])
    }
}
