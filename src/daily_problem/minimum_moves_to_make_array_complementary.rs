struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let mut count = vec![0; (2 * limit + 2) as usize];
        for i in 0..n / 2 {
            let mmin = nums[i].min(nums[n - 1 - i]);
            let mmax = nums[i].max(nums[n - 1 - i]);
            count[2] += 2;
            count[(mmin + 1) as usize] -= 1;
            count[(mmin + mmax) as usize] -= 1;
            count[(mmin + mmax + 1) as usize] += 1;
            count[(mmax + limit + 1) as usize] += 1;
        }

        let mut moves = n as i32;
        let mut cnt = 0;
        for i in 2..=2 * limit as usize {
            cnt += count[i];
            moves = moves.min(cnt);
        }
        moves
    }
}
