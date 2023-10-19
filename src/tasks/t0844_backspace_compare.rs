// https://leetcode.com/problems/backspace-string-compare

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::format(s) == Self::format(t)
    }

    fn format(s: String) -> Vec<u8> {
        let mut d = s.into_bytes();
        let mut dst = 0;
        let mut src = 0;

        while src < d.len() {
            if d[src] == b'#' {
                if dst != 0 {
                    dst -= 1;
                }
            } else {
                d[dst] = d[src];
                dst += 1;
            }

            src += 1;
        }

        d.resize(dst, 0);
        return d;
    }
}

use super::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let ans = Solution::backspace_compare(String::from("ab#c"), String::from("ac"));
        assert!(ans)
    }
}
