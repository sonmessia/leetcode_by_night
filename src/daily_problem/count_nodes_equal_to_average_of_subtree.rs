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
    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (i32, i32) {
        match root {
            Some(node) => {
                let n = node.borrow();
                let (left_sum, left_count) = Self::dfs(&n.left, ans);
                let (right_sum, right_count) = Self::dfs(&n.right, ans);
                let (sum, count) = (left_sum + right_sum + n.val, left_count + right_count + 1);
                if n.val == (sum / count) {
                    *ans += 1;
                }
                (sum, count)
            }
            None => (0, 0),
        }
    }

    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(&root, &mut ans);
        ans
    }
}
