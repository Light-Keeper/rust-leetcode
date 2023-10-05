// https://leetcode.com/problems/majority-element-ii/

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let level = nums.len() / 3 + 1;
        let mut ans = Vec::with_capacity(2);

        let mut c = 1;
        let mut last = nums[0];
        if level == 1 {
            ans.push(last)
        }

        for &x in nums[1..].iter() {
            if x != last {
                c = 0;
            }
            c += 1;
            last = x;
            
            if level == c {
                ans.push(x)
            }
        }

        return ans;
    }
}

use super::*;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let ans = Solution::majority_element(vec![1, 2, 3]);
        assert_eq!(ans, vec![]);

        let ans = Solution::majority_element(vec![3, 2, 3]);
        assert_eq!(ans, vec![3]);

        let ans = Solution::majority_element(vec![1]);
        assert_eq!(ans, vec![1]);

        let ans = Solution::majority_element(vec![1, 2]);
        assert_eq!(ans, vec![1, 2]);

        let ans = Solution::majority_element(vec![2,2,2,2,3,3,3,3,1,1]);
        assert_eq!(ans, vec![2, 3]);
    }
}