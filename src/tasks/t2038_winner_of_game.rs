// https://leetcode.com/problems/remove-colored-pieces-if-both-neighbors-are-the-same-color

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let data = colors.as_bytes();

        let mut count: i32 = 1;
        let mut a = 0;
        let mut i = 1;
        loop {
            if i < data.len() && data[i] == data[i-1] { i += 1; count += 1; continue; }
            if count > 2 {
                count -= 2;
                if data[i-1] == b'B' { count = -count; }
                a += count;
            }
            if i == data.len() { break; }
            i += 1;
            count = 1;
        }

        return a > 0;
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::winner_of_game(String::from("AAABABB")));
        assert_eq!(false, Solution::winner_of_game(String::from("AA")));
        assert_eq!(false, Solution::winner_of_game(String::from("ABBBBBBBAAA")));
        assert_eq!(false, Solution::winner_of_game(String::from("BBAA")));
    }
}
