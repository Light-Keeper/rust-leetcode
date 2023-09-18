// https://leetcode.com/problems/decode-ways/

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let raw = s.as_bytes();
        let mut a = 1;
        let mut b = (raw[0] != b'0') as i32;
        let mut c = 0;

        for i in 1..s.len() {
            if raw[i] != b'0' {
                c += b
            }
            if raw[i - 1] == b'1' || (raw[i - 1] == b'2' && raw[i] <= b'6') {
                c += a
            }
            a = b;
            b = c;
            c = 0;
        }

        return b;
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {
        assert_eq!(2, Solution::num_decodings(String::from("12")));
        assert_eq!(3, Solution::num_decodings(String::from("226")));
        assert_eq!(0, Solution::num_decodings(String::from("06")));
        assert_eq!(2, Solution::num_decodings(String::from("227")));
        assert_eq!(2, Solution::num_decodings(String::from("237")));
        assert_eq!(1, Solution::num_decodings(String::from("777")));
        assert_eq!(1, Solution::num_decodings(String::from("999999999")));
    }
}
