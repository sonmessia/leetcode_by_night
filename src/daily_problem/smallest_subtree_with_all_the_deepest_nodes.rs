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
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>,
    ) -> Option<std::rc::Rc<std::cell::RefCell<TreeNode>>> {
        fn helper(node: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
            if node.is_none() {
                return (None, 0);
            }

            let node_ref = node.as_ref().unwrap().borrow();
            let (left_node, left_depth) = helper(node_ref.left.clone());
            let (right_node, right_depth) = helper(node_ref.right.clone());

            if left_depth > right_depth {
                (left_node, left_depth + 1)
            } else if right_depth > left_depth {
                (right_node, right_depth + 1)
            } else {
                (node.clone(), left_depth + 1)
            }
        }

        let (result_node, _) = helper(root);
        result_node
    }
}
