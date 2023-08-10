
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        assert!(1 <= n && n <= 19);
        let mut buffer = [0; 20];
        buffer[0] = 1;

        for i in 0..=n as usize {
            for j in 0..i {
                buffer[i] += buffer[j]*buffer[i-j-1]
            }
        }

        return buffer[n as usize];
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_trees(3), 5);
        assert_eq!(Solution::num_trees(4), 14);
        assert_eq!(Solution::num_trees(1), 1);
    }
}