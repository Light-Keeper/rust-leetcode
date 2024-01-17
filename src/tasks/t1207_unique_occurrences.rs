// https://leetcode.com/problems/unique-number-of-occurrences
// t1207_unique_occurrences

use super::*;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut acc = HashMap::new();
        for x in arr {
            acc.entry(x).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut known = HashSet::new();
        for count in acc.values() {
            if known.contains(count) {
                return false;
            }

            known.insert(count);
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::unique_occurrences(vec![1, 2, 2, 3, 3, 3]));
        assert!(!Solution::unique_occurrences(vec![1, 2, 3]));
    }
}
