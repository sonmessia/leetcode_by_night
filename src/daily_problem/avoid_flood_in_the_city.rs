struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let n = rains.len();
        let mut ans = vec![-1; n];
        let mut lake_map = std::collections::HashMap::new();
        let mut dry_days = std::collections::BTreeSet::new();

        for (i, &rain) in rains.iter().enumerate() {
            if rain == 0 {
                dry_days.insert(i);
                ans[i] = 1; // Default value for dry days
            } else {
                if let Some(&last_rain_day) = lake_map.get(&rain) {
                    // Find the first dry day after the last rain day
                    if let Some(&dry_day) = dry_days.range(last_rain_day..).next() {
                        ans[dry_day] = rain; // Dry this lake on the dry day
                        dry_days.remove(&dry_day); // Remove this dry day from available days
                    } else {
                        return vec![]; // No available dry day to prevent flood
                    }
                }
                lake_map.insert(rain, i); // Update the last rain day for this lake
            }
        }

        ans
    }
}

fn main() {
    let rains = vec![1, 2, 0, 0, 2, 1];
    let result = Solution::avoid_flood(rains);
    println!("{:?}", result); // Output: [-1, -1, 2, 1, -1, -1]
}
