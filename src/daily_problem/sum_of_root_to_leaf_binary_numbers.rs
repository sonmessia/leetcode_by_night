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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, k: i32, ans: &mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let k = k * 2 + node.val;
            if node.left.is_none() && node.right.is_none() {
                *ans += k;
                return;
            }
            Self::dfs(node.left.clone(), k, ans);
            Self::dfs(node.right.clone(), k, ans);
        }
    }
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(root, 0, &mut ans);
        ans
    }
}

