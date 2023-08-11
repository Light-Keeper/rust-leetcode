use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        assert!(1 <= n && n <= 8);
        let mut buffer : [[_; 9]; 9] = Default::default();

        for j in 0..=n as usize {
            buffer[0][j] = vec![None];
            for i in 1..=j {
                let mut current = vec![];
                for t in 0..i {
                    for l in buffer[t][j - i + t].iter() {
                        for r in buffer[i-t-1][j].iter() {
                            current.push(Some(Rc::new(RefCell::new(TreeNode {
                                val: (j - i + t) as i32 + 1,
                                left: l.clone(),
                                right: r.clone()
                            }))))
                        }
                    }
                }
                buffer[i][j] = current
            }
        }

        mem::take(&mut buffer[n as usize][n as usize])
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = Solution::generate_trees(3);

        for x in t.iter() {
            println!("{:?}", x.as_ref().unwrap())
        }
    }
}