// https://leetcode.com/problems/frequency-of-the-most-frequent-element/?envType=daily-question&envId=2023-11-18

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut last = 0;
        let mut currnt = 0;
        let mut usage = 0;
        let mut ans = 1;

        loop {
            currnt += 1;
            if currnt == nums.len() {
                break;
            }

            usage += (nums[currnt] - nums[currnt - 1]) * ((currnt - last) as i32);
            while usage > k {
                usage -= nums[currnt] - nums[last];
                last += 1;
            }
            ans = ans.max(currnt - last + 1)
        }

        return ans as i32;
    }
}

use super::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::max_frequency(vec![1, 2, 3], 4))
    }
}
