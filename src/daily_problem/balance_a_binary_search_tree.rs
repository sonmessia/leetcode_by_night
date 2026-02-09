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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nums = vec![];

        fn convert(node: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(n) = node {
                convert(&n.borrow().left, nums);
                nums.push(n.borrow().val);
                convert(&n.borrow().right, nums);
            }
        }

        fn build(nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }

            let mid = nums.len() / 2;

            let node = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            node.borrow_mut().left = build(&nums[..mid].to_vec());
            node.borrow_mut().right = build(&nums[mid + 1..].to_vec());
            Some(node)
        }

        convert(&root, &mut nums);
        build(&nums)
    }
}
