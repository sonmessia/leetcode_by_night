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
    fn find_perfect_subtrees(node: &Option<Rc<RefCell<TreeNode>>>, sizes: &mut Vec<i32>) -> i32 {
        match node {
            Some(n) => {
                let left_size = Self::find_perfect_subtrees(&n.borrow().left, sizes);
                let right_size = Self::find_perfect_subtrees(&n.borrow().right, sizes);

                if left_size >= 0 && right_size >= 0 && left_size == right_size {
                    let subtree_size = left_size + right_size + 1;
                    sizes.push(subtree_size);
                    subtree_size
                } else {
                    -1
                }
            }
            None => 0,
        }
    }
    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut sizes = vec![];

        Self::find_perfect_subtrees(&root, &mut sizes);

        if sizes.len() < k as usize {
            return -1;
        }
        sizes.sort_unstable_by(|a, b| b.cmp(a));

        sizes.get((k - 1) as usize).unwrap().to_owned()
    }
}
