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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;

        let mut max_sum = i32::MIN;
        let mut answer_level = i32::MAX;
        let mut current_level = 1;
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level_sum = 0;

            for _ in 0..level_size {
                if let Some(Some(node_rc)) = queue.pop_front() {
                    let node = node_rc.borrow();
                    current_level_sum += node.val;

                    if node.left.is_some() {
                        queue.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.clone());
                    }
                }
            }

            if current_level_sum > max_sum {
                max_sum = current_level_sum;
                answer_level = current_level;
            }
            current_level += 1;
        }

        answer_level
    }
}
