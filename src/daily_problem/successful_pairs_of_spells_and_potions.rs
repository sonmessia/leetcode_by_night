struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        fn binary_search(potions: &Vec<i32>, spell: i32, success: i64) -> usize {
            let mut left = 0;
            let mut right = potions.len();
            while left < right {
                let mid = left + (right - left) / 2;
                if (spell as i64) * (potions[mid] as i64) >= success {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        }
        let mut potions = potions;
        potions.sort_unstable();
        let mut result = Vec::with_capacity(spells.len());
        for &spell in &spells {
            let idx = binary_search(&potions, spell, success);
            result.push((potions.len() - idx) as i32);
        }
        result
    }
}
