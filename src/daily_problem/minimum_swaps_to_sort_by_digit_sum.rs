struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let sum_of_digits = |mut n: i32| -> i32 {
            let mut total = 0;
            while n > 0 {
                total += n % 10;
                n /= 10;
            }
            total
        };

        let n = nums.len();
        let mut map: HashMap<i32, (i32, usize)> = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            map.insert(*num, (sum_of_digits(*num), idx));
        }

        // println!("{:?}", map);

        let mut sorted_nums = nums.clone();

        sorted_nums.sort_by(|a, b| map[a].0.cmp(&map[b].0).then_with(|| a.cmp(b)));

        // println!("{:?}", sorted_nums);

        let mut ans = 0;

        let mut visited = vec![false; n];

        for i in 0..n {
            if visited[i] || sorted_nums[i] == nums[i] {
                continue;
            }

            let mut cycle_size = 0;
            let mut j = i;

            while !visited[j] {
                visited[j] = true;
                j = map[&sorted_nums[j]].1;
                cycle_size += 1;
            }

            if cycle_size > 0 {
                ans += cycle_size - 1;
            }
        }

        ans
    }
}
