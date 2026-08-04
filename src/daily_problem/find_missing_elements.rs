struct Solution;

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let (max_v, min_v) = match (nums.iter().max(), nums.iter().min()) {
            (Some(max), Some(min)) => (*max, *min),
            _ => return vec![],
        };

        let mut present = std::collections::HashSet::new();
        for &num in &nums {
            present.insert(num);
        }

        let mut ans = vec![];
        for i in min_v..=max_v {
            if !present.contains(&i) {
                ans.push(i);
            }
        }

        ans
    }
}
