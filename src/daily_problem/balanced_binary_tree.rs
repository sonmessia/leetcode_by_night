// Definition for a binary tree node.
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
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => -1,
            Some(n) => {
                let n = n.borrow();
                1 + Self::height(&n.left).max(Self::height(&n.right))
            }
        }
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,

            Some(node) => {
                let node_ref = node.borrow();

                (Self::height(&node_ref.left) - Self::height(&node_ref.right)).abs() <= 1
                    && Self::is_balanced(node_ref.left.clone())
                    && Self::is_balanced(node_ref.right.clone())
            }
        }
    }
}
