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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::generate_trees_helper(1, n)
    }

    fn generate_trees_helper(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None];
        }

        let mut all_trees = Vec::new();

        for i in start..=end {
            let left_trees = Self::generate_trees_helper(start, i - 1);
            let right_trees = Self::generate_trees_helper(i + 1, end);

            for left in &left_trees {
                for right in &right_trees {
                    let root = Rc::new(RefCell::new(TreeNode::new(i)));
                    root.borrow_mut().left = left.clone();
                    root.borrow_mut().right = right.clone();
                    all_trees.push(Some(root));
                }
            }
        }

        println!(
            "Generated {} trees for range {} to {}",
            all_trees.len(),
            start,
            end
        );

        all_trees
    }
}
