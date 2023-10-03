// https://leetcode.com/problems/symmetric-tree/

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl Default for TreeNode {
    fn default() -> Self {
        Self { val: Default::default(), left: Default::default(), right: Default::default() }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let TreeNode { right, left, .. } = root.unwrap().take();
        Solution::sync_nodes_review(right, left)
    }

    fn sync_nodes_review(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if a.is_none() && b.is_none() {
            return true;
        }

        if a.is_none() || b.is_none() {
            return false;
        }
    
        let t1 = a.unwrap().take();
        let t2 = b.unwrap().take();
        
        if t1.val != t2.val {
            return false;
        }

        return Self::sync_nodes_review(t1.left, t2.right) && Self::sync_nodes_review(t1.right,  t2.left);
    }
}

use super::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let a = Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None}));
        let b = Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None}));
        let c = Rc::new(RefCell::new(TreeNode{val: 2, left: Some(a), right: Some(b)}));

        let ans = Solution::is_symmetric(Some(c));
        assert!(ans);
    }

    #[test]
    fn test2() {
        let a = Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None}));
        let b = Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None}));
        let c = Rc::new(RefCell::new(TreeNode{val: 2, left: Some(a), right: Some(b)}));

        let ans = Solution::is_symmetric(Some(c));
        assert!(!ans);
    }
}