struct Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let mut max_overlap = 0;

        for x_shift in -(n as i32 - 1)..=(n as i32 - 1) {
            for y_shift in -(n as i32 - 1)..=(n as i32 - 1) {
                let mut overlap = 0;
                for i in 0..n {
                    for j in 0..n {
                        let new_i = i as i32 + x_shift;
                        let new_j = j as i32 + y_shift;
                        if new_i >= 0 && new_i < n as i32 && new_j >= 0 && new_j < n as i32 {
                            if img1[i][j] == 1 && img2[new_i as usize][new_j as usize] == 1 {
                                overlap += 1;
                            }
                        }
                    }
                }
                max_overlap = max_overlap.max(overlap);
            }
        }

        max_overlap
    }
}
