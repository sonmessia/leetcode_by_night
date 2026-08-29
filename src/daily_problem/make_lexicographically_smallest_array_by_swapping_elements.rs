struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();

        let mut pairs: Vec<(i32, usize)> =
            nums.iter().enumerate().map(|(i, &num)| (num, i)).collect();

        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        // println!("Pairs :{:?}", pairs);

        let mut groups = vec![];

        let mut current_group = vec![pairs[0]];

        for i in 1..n {
            if (pairs[i].0 - pairs[i - 1].0).abs() <= limit {
                current_group.push(pairs[i]);
            } else {
                groups.push(current_group);
                current_group = vec![pairs[i]];
            }
        }

        groups.push(current_group);

        // println!("Groups :{:?}", groups);

        let mut ans = vec![0; n];

        for mut group in groups.into_iter() {
            group.sort_by(|a, b| a.0.cmp(&b.0));
            let sorted_num: Vec<i32> = group.iter().map(|&(num, _)| num).collect();
            group.sort_by(|a, b| a.1.cmp(&b.1));
            let sorted_indices: Vec<usize> = group.iter().map(|&(_, idx)| idx).collect();

            for (i, &idx) in sorted_indices.iter().enumerate() {
                ans[idx] = sorted_num[i];
            }
        }

        ans
    }
}
