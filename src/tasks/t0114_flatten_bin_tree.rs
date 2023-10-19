// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let result = Solution::flatten_with_suffix(root, None);
        let _ = std::mem::replace(root, result);
    }

    fn flatten_with_suffix(root: &mut Option<Rc<RefCell<TreeNode>>>, suffux: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => suffux,
            Some(x) => {
                let mut node = x.replace(TreeNode { val: 0, left: None, right: None });
                let s2 = Self::flatten_with_suffix(&mut node.right, suffux);
                let s3 = Self::flatten_with_suffix(&mut node.left, s2);
                x.replace(TreeNode { val: node.val, left: None, right: s3 });
                return Some(x.clone());
            }
        }
    }
}

#[cfg(test)] 
mod test {
    use super::*;

    #[test] 
    fn test() {
        Solution::flatten(&mut None)
    }
}