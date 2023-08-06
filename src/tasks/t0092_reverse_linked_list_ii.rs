// https://leetcode.com/problems/reverse-linked-list-ii/

impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, mut left: i32, mut right: i32) -> Option<Box<ListNode>> {
        let mut values = Vec::new();

        while let Some(node) = head {
            values.push(node.val);
            head = node.next
        }

        left -= 1;
        right -= 1;

        while left < right {
            values.swap(left as usize, right as usize);
            left += 1;
            right -= 1;
        }

        Self::slice_to_list(&values[..])
    }

    fn slice_to_list(values: &[i32]) -> Option<Box<ListNode>> {
        if values.is_empty() {
            None
        } else {
            Some(Box::new(ListNode{
                val: values[0],
                next: Self::slice_to_list(&values[1..])
            }))
        }
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::slice_to_list(&[1, 4, 3, 2, 5]),
            Solution::reverse_between(Solution::slice_to_list(&[1, 2, 3, 4 ,5]), 2, 4)
        );

        assert_eq!(
            Solution::slice_to_list(&[5]),
            Solution::reverse_between(Solution::slice_to_list(&[5]), 1, 1)
        );
    }

}