struct Solution;

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut left = vec![n as i32; n];
        let mut right = vec![0; n];
        let mut below = vec![n as i32; n];
        let mut above = vec![0; n];

        for b in &buildings {
            let (x, y) = (b[0] as usize - 1, b[1] as usize - 1);
            left[y] = left[y].min(x as i32);
            right[y] = right[y].max(x as i32);
            below[x] = below[x].min(y as i32);
            above[x] = above[x].max(y as i32);
        }
        let mut ans = 0;

        for b in buildings {
            let (x, y) = (b[0] as usize - 1, b[1] as usize - 1);
            if (x as i32) > left[y]
                && (x as i32) < right[y]
                && (y as i32) > below[x]
                && (y as i32) < above[x]
            {
                ans += 1;
            }
        }

        ans
    }
}
