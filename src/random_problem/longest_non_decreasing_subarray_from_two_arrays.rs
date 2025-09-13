struct Solution;

impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut max_length = 1;
        let (mut dp1, mut dp2) = (1, 1);
        if n == 1 {
            return 1;
        }

        for i in 1..n {
            let (mut curr1, mut curr2) = (1, 1);
            if nums1[i] >= nums1[i - 1] {
                curr1 = curr1.max(dp1 + 1);
            }
            if nums2[i] >= nums2[i - 1] {
                curr2 = curr2.max(dp2 + 1);
            }

            if nums1[i] >= nums2[i - 1] {
                curr1 = curr1.max(dp2 + 1);
            }

            if nums2[i] >= nums1[i - 1] {
                curr2 = curr2.max(dp1 + 1);
            }

            max_length = max_length.max(curr1).max(curr2);
            dp1 = curr1;
            dp2 = curr2;
        }
        max_length
    }
}

fn main() {
    let nums1 = vec![5, 16, 15];
    let nums2 = vec![12, 1, 14];
    let result = Solution::max_non_decreasing_length(nums1, nums2);
    println!("Result: {}", result); // Output: Result: 3
}
