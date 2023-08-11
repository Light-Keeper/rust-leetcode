// https://leetcode.com/problems/coin-change-ii/


impl Solution {
    pub fn change(amount: i32, mut coins: Vec<i32>) -> i32 {
        coins.sort_unstable();
        let mut buffers : Vec<VecDeque<i32>> = Default::default();

        for &val in coins.iter() {
            buffers.push(VecDeque::with_capacity(val as usize + 1));
            buffers.last_mut().unwrap().push_front(1);
        }

        for _ in 1..=amount as usize {
            let mut acc = 0;
            for t in 0..coins.len() {
                let b = &mut buffers[t];
                if b.len() == coins[t] as usize {
                    acc += b.pop_back().unwrap();
                }
                b.push_front(acc);
            }
        }

        buffers.last_mut().unwrap().pop_front().unwrap()
    }
}

use std::collections::VecDeque;
use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::change(5, vec![1,2, 5]));
        assert_eq!(0, Solution::change(3, vec![2]));
        assert_eq!(1, Solution::change(10, vec![10]));
    }
}