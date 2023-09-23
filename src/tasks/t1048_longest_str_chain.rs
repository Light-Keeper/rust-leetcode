// https://leetcode.com/problems/longest-string-chain/

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        words.sort_unstable_by(|a, b| a.len().cmp(&b.len()));

        let mut ans = 0;
        let mut chains = vec![0; words.len()];
        for i in 0..words.len() {
            chains[i] = 1;
            for j in 0..i { 
                if chains[j] >= chains[i] && Self::is_predecessor(&words[j], &words[i]) {
                    chains[i] = chains[j] + 1;
                }
            }
            ans = ans.max(chains[i])
        }
        return ans;
    }

    fn is_predecessor(a: &str, b: &str) -> bool {
        if b.len() != a.len() + 1 {
            return false;
        }

        let mut i = 0;
        while i < a.len() && a.as_bytes()[i] == b.as_bytes()[i] {
            i += 1;
        }

        while i < a.len() && a.as_bytes()[i] == b.as_bytes()[i + 1] {
            i += 1;
        }

        return i == a.len();
    }
}


use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        assert_eq!(Solution::longest_str_chain(vec!["a","b","ba","bca","bda","bdca"].into_iter().map(|s| s.to_string()).collect()), 4);
        assert_eq!(Solution::longest_str_chain(vec!["xbc","pcxbcf","xb","cxbc","pcxbc"].into_iter().map(|s| s.to_string()).collect()), 5);
    }
}
