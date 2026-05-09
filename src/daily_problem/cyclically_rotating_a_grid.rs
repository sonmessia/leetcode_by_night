struct Solution;

impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut top_left, mut top_right, mut bottom_left, mut bottom_right) = ((0, 0), (0, n - 1), (m - 1, 0), (m - 1, n - 1));
        
        let len = (bottom_left.0 - top_left.0 + 1) * 2 + (top_right.1 - top_left.1 - 1) * 2;
        let mut tmp: Vec<i32> = Vec::with_capacity(len);
        while top_left.0 < bottom_left.0 && top_left.1 < top_right.1 {
            let len = (bottom_left.0 - top_left.0 + 1) * 2 + (top_right.1 - top_left.1 - 1) * 2;
            let k_usize = (k as usize) % len;

            for j in top_left.1..=top_right.1 {
                tmp.push(grid[top_left.0][j]);
            }
            for i in top_left.0 + 1..=bottom_left.0 {
                tmp.push(grid[i][top_right.1]);
            }
            for j in (top_left.1..top_right.1).rev() {
                tmp.push(grid[bottom_left.0][j]);
            }
            for i in (top_left.0 + 1..bottom_left.0).rev() {
                tmp.push(grid[i][top_left.1]);
            }

            println!("tmp = {:?}", tmp);
            tmp.rotate_left(k_usize);
            println!("tmp = {:?}", tmp);
            for j in top_left.1..=top_right.1 {
                grid[top_left.0][j] = tmp.remove(0);
            }
            for i in top_left.0 + 1..=bottom_left.0 {
                grid[i][top_right.1] = tmp.remove(0);
            }
            for j in (top_left.1..top_right.1).rev() {
                grid[bottom_left.0][j] = tmp.remove(0);
            }
            for i in (top_left.0 + 1..bottom_left.0).rev() {
                grid[i][top_left.1] = tmp.remove(0);
            }  
            
            println!("{:?} {:?} {:?} {:?}", top_left, top_right, bottom_left, bottom_right);
            println!("len = {}", len);
            
            top_left = (top_left.0 + 1, top_left.1 + 1);
            top_right = (top_right.0 + 1, top_right.1 - 1);
            bottom_left = (bottom_left.0 - 1, bottom_left.1 + 1);
            bottom_right = (bottom_right.0 - 1, bottom_right.1 - 1);
        }

        grid       
    }
}
