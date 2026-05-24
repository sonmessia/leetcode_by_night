struct Solution;

impl Solution {
    fn dfs(arr: &Vec<i32>, mut dp: &mut Vec<i32>, i: i32, d: usize) -> i32 {
        if dp[i as usize] != 0 {
            return dp[i as usize];
        }

        let mut j = i - 1;
        let mut max_val = 1;
        while j >= 0 && (i - j).abs() as usize <= d && arr[j as usize] < arr[i as usize] {
            max_val = max_val.max(Self::dfs(arr, &mut dp, j, d) + 1);
            if j == 0 {
                break;
            }
            j -= 1;
        }

        j = i + 1;
        for j in j as usize..arr.len() {
            if (j as i32 - i).abs() as usize > d || arr[j as usize] >= arr[i as usize] {
                break;
            }
            max_val = max_val.max(Self::dfs(arr, &mut dp, j as i32, d) + 1);
        }

        dp[i as usize] = max_val;
        dp[i as usize]
    }
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len();
        let mut dp = vec![0; n];
        let d_u = d as usize;

        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(Self::dfs(&arr, &mut dp, i as i32, d_u));
        }
        ans
    }
}
