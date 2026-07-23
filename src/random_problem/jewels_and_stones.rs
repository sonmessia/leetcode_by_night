struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut map = std::collections::HashMap::new();

        let mut ans = 0;

        for ch in stones.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        for ch in jewels.chars() {
            if let Some(&count) = map.get(&ch) {
                ans += count;
            }
        }

        ans
    }
}
