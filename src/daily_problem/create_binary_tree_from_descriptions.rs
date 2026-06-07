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
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node_map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut children: HashSet<i32> = HashSet::new();

        for desc in descriptions {
            let parent_val = desc[0];
            let child_val = desc[1];
            let is_left = desc[2] == 1;

            node_map
                .entry(parent_val)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent_val))));
            node_map
                .entry(child_val)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child_val))));

            let parent_node = node_map.get(&parent_val).unwrap();
            let child_node = node_map.get(&child_val).unwrap();

            if is_left {
                parent_node.borrow_mut().left = Some(Rc::clone(child_node));
            } else {
                parent_node.borrow_mut().right = Some(Rc::clone(child_node));
            }

            children.insert(child_val);
        }

        for (parent_val, node) in node_map {
            if !children.contains(&parent_val) {
                return Some(node);
            }
        }

        None
    }
}
