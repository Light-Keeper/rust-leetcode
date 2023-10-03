// https://leetcode.com/problems/insertion-sort-list/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut i = &head.as_ref().unwrap().next;

        while i.is_some() {
            let mut j = &head;
            while j.as_ref().unwrap().val < i.as_ref().unwrap().val {
                j = &j.as_ref().unwrap().next
            }

            if i != j { 
                unsafe {
                    let next_ref = &i.as_ref().unwrap().next;
                    let a = (j as *const Option<Box<ListNode>>) as *mut Option<Box<ListNode>>;
                    let c = (i as *const Option<Box<ListNode>>) as *mut Option<Box<ListNode>>;
                    let b = (next_ref as *const Option<Box<ListNode>>) as *mut Option<Box<ListNode>>;

                    std::ptr::swap(a, b);
                    std::ptr::swap(a, c);
                }                
            } else {
                i = &i.as_ref().unwrap().next;
            }    
        }

        return head;
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = Solution::insertion_sort_list(Some(Box::new(ListNode{ val: 1, next: None })));
        assert_eq!(result, Some(Box::new(ListNode{ val: 1, next: None })));

        let result = Solution::insertion_sort_list(Some(Box::new(ListNode{ val: 1, next: Some(Box::new(ListNode { val: -1, next: None })) })));
        assert_eq!(result, Some(Box::new(ListNode{ val: -1, next: Some(Box::new(ListNode { val: 1, next: None })) })));
    }
}
