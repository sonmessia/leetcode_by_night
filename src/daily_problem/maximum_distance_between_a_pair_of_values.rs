struct Solution;

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let (m, n) = (nums1.len(), nums2.len());

        for j in 0..n {
            while i < m && nums1[i] > nums2[j] {
                i += 1;
            }

            if j >= i && i < m {
                ans = ans.max(j as i32 - i as i32);
            }
        }
        ans
    }
}
