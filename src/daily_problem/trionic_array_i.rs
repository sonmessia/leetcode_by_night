struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let mut p = 0;

        for i in 1..n {
            if nums[i] <= nums[i - 1] {
                p = i - 1;
                break;
            }
        }

        if p == 0 {
            return false;
        }

        let mut q = p;

        for i in p + 1..n {
            if nums[i] >= nums[i - 1] {
                q = i - 1;
                break;
            }
        }

        if p == q {
            return false;
        }

        for i in q + 1..n {
            if nums[i] <= nums[i - 1] {
                return false;
            }
        }

        true
    }
}
