// https://leetcode.com/problems/evaluate-reverse-polish-notation/

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for t in tokens.into_iter() {
            let x : i32 = if t.as_str() == "-" {
                - stack.pop().unwrap() + stack.pop().unwrap()
            } else if t.as_str() == "+" {
                stack.pop().unwrap() + stack.pop().unwrap()
            } else if t.as_str() == "*" {
                stack.pop().unwrap() * stack.pop().unwrap()
            } else if t.as_str() == "/" {
                let a = stack.pop().unwrap();
                stack.pop().unwrap() / a
            } else {
                t.parse().unwrap()
            };

            stack.push(x)
        }

        stack.pop().unwrap()
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(9, Solution::eval_rpn(vec!["2","1","+","3","*"].into_iter().map(String::from).collect()));
        assert_eq!(6, Solution::eval_rpn(vec!["4","13","5","/","+"].into_iter().map(String::from).collect()));
        assert_eq!(22, Solution::eval_rpn(vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"].into_iter().map(String::from).collect()));
    }
}