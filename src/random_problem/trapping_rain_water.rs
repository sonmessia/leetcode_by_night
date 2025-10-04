struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 1 {
            return 0;
        }
        let mut ans = 0;

        let mut curr_sum = 0;

        let (mut i, mut j) = (0, n - 1);
        while i < (n - 1) && height[i] <= height[i + 1] {
            i += 1;
        }

        while j > 0 && height[j] <= height[j - 1] {
            j -= 1;
        }
        // println!("{} {}", i, j);

        if i == n - 1 || j == 0 {
            return 0;
        }

        for k in (i + 1)..=j {
            if height[k] >= height[i] {
                ans += ((k as i32 - i as i32 - 1) * height[i]) - curr_sum;
                i = k;
                curr_sum = 0;
            } else {
                curr_sum += height[k];
            }
        }

        curr_sum = 0;

        println!("{} {}", i, j);

        for k in (i..j).rev() {
            if height[k] >= height[j] {
                ans += ((j as i32 - k as i32 - 1) * height[j]) - curr_sum;
                j = k;
                curr_sum = 0;
            } else {
                curr_sum += height[k];
            }
        }
        ans
    }
}

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let ans = Solution::trap(height);
    println!("{}", ans);

    let height = vec![4, 2, 0, 3, 2, 5];
    let ans = Solution::trap(height);
    println!("{}", ans);

    let height = vec![4, 2, 3];
    let ans = Solution::trap(height);
    println!("{}", ans);
}
