// https://leetcode.com/problems/valid-palindrome/
// t0125_is_palindrome

use super::*;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.into_bytes();
        let i1 = Self::filtered_iterator(bytes.iter().copied());
        let i2 = Self::filtered_iterator(bytes.iter().rev().copied());
        i1.eq(i2)
    }

    fn filtered_iterator(src: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
        src.filter(|&c| (b'a' <= c && c <= b'z') || (b'A' <= c && c <= b'Z') || (b'0' <= c && c <= b'9'))
            .map(|c| {
                if b'A' <= c && c <= b'Z' {
                    c - b'A' + b'a'
                } else {
                    c
                }
            })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]

    pub fn test() {
        assert!(Solution::is_palindrome(String::from("acbca")));
        assert!(Solution::is_palindrome(String::from(
            "A man, a plan, a canal: Panama"
        )));
    }
}
