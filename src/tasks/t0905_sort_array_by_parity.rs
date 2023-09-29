// https://leetcode.com/problems/sort-array-by-parity/

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = nums.len() - 1;
        
        while i < j {
            while nums[i] % 2 == 0 && i < j { i += 1; }    
            while nums[j] % 2 == 1 && i < j { j -= 1; }    
            nums.swap(i, j)
        }

        return nums;
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {
        assert_eq!(Solution::sort_array_by_parity(vec![3,1,2,4]), vec![4,2,1,3])
    }
}