
// https://leetcode.com/problems/find-the-duplicate-number/

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut start = 0usize;
        let next = |i: usize| { nums[i] as usize - 1 };
        
        let is_reachable = |from: usize, to: usize| {
            let mut i = next(from);
            while i != from && i != to {
                i = next(i);
            }

            return i == to;
        };

        loop {
            let mut i = next(start);
            let mut j = next(i);
            while i != j && i >= start {
                i = next(i);
                j = next(next(j));
            }

            if i < start { 
                j = i;
            }


            if is_reachable(i, start) {
                start += 1;
                continue;
            }
            
            let mut a: usize = 0;
            let mut b: usize = 1;
            i = start;
            while i != j {
                b += 1;
                i = next(i);
            }            
            
            while a + 1 != b {
                let c = (a + b) / 2;
                i = start;
                for _ in 0..c {
                    i = next(i);
                }
                if is_reachable(j, i) {
                    b = c;
                } else {
                    a = c;
                }
            }

            i = start;
            for _ in 0..a {
                i = next(i);
            }

            return nums[i];
        }
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = Solution::find_duplicate(vec![1, 3, 4, 2, 2]);
        assert_eq!(result, 2);

        let result = Solution::find_duplicate(vec![3, 1, 3, 4, 2]);
        assert_eq!(result, 3);

        let result = Solution::find_duplicate(vec![2, 2, 2, 2]);
        assert_eq!(result, 2);

        let result = Solution::find_duplicate(vec![1, 2, 2, 2]);
        assert_eq!(result, 2);

        let result = Solution::find_duplicate(vec![1, 2, 2, 3]);
        assert_eq!(result, 2);
    }
}
