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
impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut ans: i64 = 0;

        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            total_sum: i64,
            ans: &mut i64,
            compute_ans: bool,
        ) -> i64 {
            if let Some(node) = root {
                let node_borrow = node.borrow();
                let left_sum = dfs(&node_borrow.left, total_sum, ans, compute_ans);
                let right_sum = dfs(&node_borrow.right, total_sum, ans, compute_ans);
                let subtree_sum = left_sum + right_sum + node_borrow.val as i64;
                if compute_ans {
                    let product = subtree_sum * (total_sum - subtree_sum);
                    *ans = std::cmp::max(*ans, product);
                }
                subtree_sum
            } else {
                0
            }
        }

        let total_sum = dfs(&root, 0, &mut ans, false);
        println!("Total sum: {}", total_sum);
        dfs(&root, total_sum, &mut ans, true);
        (ans % MOD) as i32
    }
}
