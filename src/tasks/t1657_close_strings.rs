// https://leetcode.com/problems/determine-if-two-strings-are-close
// t1657_close_strings
use super::*;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        return Self::letters(&word1) == Self::letters(&word2)
            && Self::counters(&word1) == Self::counters(&word2);
    }

    fn letters(workd: &String) -> [bool; 30] {
        let mut ans = [false; 30];
        workd
            .as_bytes()
            .iter()
            .for_each(|&byte| ans[(byte - b'a') as usize] = true);
        return ans;
    }

    fn counters(workd: &String) -> [usize; 30] {
        let mut ans = [0; 30];
        workd
            .as_bytes()
            .iter()
            .for_each(|&byte| ans[(byte - b'a') as usize] += 1);
        ans.sort_unstable();
        return ans;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]

    fn test() {
        assert!(Solution::close_strings(
            String::from("abc"),
            String::from("bca")
        ));

        assert!(Solution::close_strings(
            String::from("cabbba"),
            String::from("abbccc")
        ));

        assert!(!Solution::close_strings(
            String::from("a"),
            String::from("aa")
        ))
    }
}
