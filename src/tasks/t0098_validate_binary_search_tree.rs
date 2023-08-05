// https://leetcode.com/problems/validate-binary-search-tree/

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Range;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_with_constraint(i64::MIN..i64::MAX, root)
    }

    fn is_valid_with_constraint(range: Range<i64>, node: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return match node {
            None => {
                true
            }

            Some(reference) => {
                let val = reference.replace(TreeNode::new(0));
                if !range.contains(&(val.val as i64)) {
                    false
                } else {
                    Self::is_valid_with_constraint(range.start..val.val as i64, val.left) &&
                        Self::is_valid_with_constraint(((val.val as i64) + 1)..range.end, val.right)
                }
            }
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_3_nodes() {
        let a = Rc::new(RefCell::new(TreeNode::new(1)));
        let c = Rc::new(RefCell::new(TreeNode::new(3)));
        let b = Rc::new(RefCell::new(TreeNode::new(2)));

        b.borrow_mut().left = Some(a);
        b.borrow_mut().right = Some(c);

        assert_eq!(Solution::is_valid_bst(Some(b)), true);
    }
}