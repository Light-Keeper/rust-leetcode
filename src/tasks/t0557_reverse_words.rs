// https://leetcode.com/problems/reverse-words-in-a-string-iii/

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut bytes = s.into_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b' ' { i += 1; continue; } 
            let mut j = i;
            while j < bytes.len() && bytes[j] != b' ' { j += 1; continue; }
            let t = j;
            j -= 1;
            while i < j { bytes.swap(i, j); i += 1; j -= 1; }
            i = t + 1
        }

        return unsafe { String::from_utf8_unchecked(bytes) } ;
    }
}


use super::*;

#[cfg(test)] 
mod test {
    use super::*;

    #[test]
    pub fn test() {
        let s = Solution::reverse_words("hello here".into());
        assert_eq!(s, "olleh ereh")
    }
}
