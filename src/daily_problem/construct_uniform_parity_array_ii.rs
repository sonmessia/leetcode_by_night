struct Solution;

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        if nums1.iter().all(|&x| x % 2 == 0) || nums1.iter().all(|&x| x % 2 != 0) {
            return true;
        }

        let min_num = nums1.iter().min().unwrap_or(&0);

        if min_num & 1 == 1 {
            for num in nums1.iter() {
                if num & 1 == 0 && num - min_num < 1 {
                    return false;
                }
            }
        } else {
            for num in nums1.iter() {
                if num & 1 == 1 && num - min_num < 1 || (num - min_num) & 1 == 1 {
                    return false;
                }
            }
        }

        true
    }
}
