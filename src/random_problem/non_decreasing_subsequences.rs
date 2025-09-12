use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_subsequences(num: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(i: usize, ans: &mut HashSet<Vec<i32>>, t: &mut Vec<i32>, nums: &Vec<i32>) {
            if t.len() >= 2 {
                ans.insert(t.clone());
            }
            for j in i..nums.len() {
                if t.is_empty() || (*t.last().unwrap() <= nums[j]) {
                    t.push(nums[j]);
                    backtrack(j + 1, ans, t, nums);
                    t.pop();
                }
            }
        }
        let mut ans = HashSet::new();
        let mut t = vec![];
        backtrack(0, &mut ans, &mut t, &num);
        ans.into_iter().collect()
    }
}

fn main() {
    let result = Solution::find_subsequences(vec![4, 6, 7, 7]);
    println!("{:?}", result);
}
