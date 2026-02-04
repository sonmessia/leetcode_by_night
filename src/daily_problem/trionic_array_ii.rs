struct Solution;

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut p: usize;
        let mut q: usize;
        let mut j: usize;
        let mut max_sum: i64;
        let mut sum: i64;
        let mut res: i64;
        let mut ans: i64 = i64::MIN;
        let mut i = 0;
        while i < n {
            j = i + 1;
            res = 0;
            while j < n && nums[j - 1] < nums[j] {
                j += 1;
            }
            p = j - 1;
            if p == i {
                i += 1;
                continue;
            }
            res += (nums[p] + nums[p - 1]) as i64;
            while j < n && nums[j - 1] > nums[j] {
                res += nums[j] as i64;
                j += 1;
            }
            q = j - 1;
            if q == p || q == n - 1 || (j < n && nums[j] <= nums[q]) {
                i = q;
                continue;
            }
            res += nums[q + 1] as i64;
            max_sum = 0;
            sum = 0;
            let mut k = q + 2;
            while k < n && nums[k] > nums[k - 1] {
                sum += nums[k] as i64;
                max_sum = max_sum.max(sum);
                k += 1;
            }
            res += max_sum;
            max_sum = 0;
            sum = 0;
            let mut k = p as isize - 2;
            while k >= i as isize {
                sum += nums[k as usize] as i64;
                max_sum = max_sum.max(sum);
                k -= 1;
            }
            res += max_sum;
            ans = ans.max(res);
            i = q - 1;
        }
        ans
    }
}

fn main() {
    let nums = vec![1, 6, 6, 3, 7];
    println!("{}", Solution::max_sum_trionic(nums));
}
