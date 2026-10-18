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

use std::collections::HashMap;

struct Solution;

impl Solution {
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<String, (Option<Rc<RefCell<TreeNode>>>, usize)>,
        ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> String {
        if let Some(node) = root {
            let left = Solution::dfs(node.borrow().left.clone(), map, ans);
            let right = Solution::dfs(node.borrow().right.clone(), map, ans);
            let serial = format!("{},{},{}", node.borrow().val, left, right);
            if let Some((_, count)) = map.get_mut(&serial) {
                *count += 1;
                if *count == 2 {
                    ans.push(Some(node.clone()));
                }
            } else {
                map.insert(serial.clone(), (Some(node.clone()), 1));
            }
            serial
        } else {
            "#".to_string()
        }
    }
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut map: HashMap<String, (Option<Rc<RefCell<TreeNode>>>, usize)> = HashMap::new();
        let mut ans: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

        Self::dfs(root, &mut map, &mut ans);

        // println!("map: {:?}", map.keys());

        ans.into_iter().collect()
    }
}
