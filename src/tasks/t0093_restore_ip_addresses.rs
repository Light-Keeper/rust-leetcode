// https://leetcode.com/problems/restore-ip-addresses/

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut vec = s.into_bytes();
        vec.iter_mut().for_each(|v| *v = *v - b'0');

        Self::find_ways_to_split_digits(
            &vec[..],
            4,
            &mut [0u8, 0u8, 0u8, 0u8],
            0,
            &mut result
        );

        return result
    }

    fn find_ways_to_split_digits(src: &[u8], segments: u8, stack: &mut [u8], sp: usize, dst: &mut Vec<String>) {
        if sp == 4 {
            if src.is_empty() {
                dst.push(format!("{}.{}.{}.{}", stack[0], stack[1], stack[2], stack[3]))
            }
            return;
        }

        // fast track on too evidently wrong data
        let max_scr_len = (segments * 3) as usize;
        let min_src_len = (segments * 1) as usize;

        if src.len() < min_src_len || src.len() > max_scr_len {
            return;
        }

        // 1-digit case: any digit is valid, len != 0
        stack[sp] = src[0];
        Self::find_ways_to_split_digits(&src[1..], segments-1, stack, sp+1, dst);

        // 2-digit case: 0x is not valid, anything else is valid
        if src.len() >= 2 && src[0] != 0 {
            stack[sp] = src[0] * 10 + src[1];
            Self::find_ways_to_split_digits(&src[2..], segments-1, stack, sp+1, dst);
        }

        // 3-digit case: 0xx is not valid, the value must be less than 256
        if src.len() >= 3 && src[0] != 0 {
            let val = src[0] as i32 * 100 + src[1] as i32 * 10 + src[2] as i32;
            if val < 256 {
                stack[sp] = val as u8;
                Self::find_ways_to_split_digits(&src[3..], segments-1, stack, sp+1, dst);
            }
        }
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {
        assert_eq!(
            Solution::restore_ip_addresses(String::from("25525511135")),
            vec!["255.255.11.135","255.255.111.35"]
        );

        assert_eq!(
            Solution::restore_ip_addresses(String::from("0000")),
            vec!["0.0.0.0"]
        );

        assert_eq!(
            Solution::restore_ip_addresses(String::from("101023")),
            vec!["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
        );
    }
}
