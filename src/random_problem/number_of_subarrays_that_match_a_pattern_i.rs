struct Solution;

impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let (m, n) = (pattern.len(), nums.len());
        let mut ans = 0;

        for i in 0..(n - m) {
            let start = i;
            let mut is_match = true;

            for (idx, &p) in pattern.iter().enumerate() {
                match p {
                    -1 => {
                        if nums[start + idx + 1] >= nums[start + idx] {
                            is_match = false;
                            break;
                        }
                    }
                    0 => {
                        if nums[start + idx + 1] != nums[start + idx] {
                            is_match = false;
                            break;
                        }
                    }
                    _ => {
                        if nums[start + idx + 1] <= nums[start + idx] {
                            is_match = false;
                            break;
                        }
                    }
                }
            }

            println!("start: {}, is_match: {}", start, is_match);

            ans += if is_match { 1 } else { 0 };
        }

        ans
    }
}
