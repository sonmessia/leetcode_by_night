struct Solution;

impl Solution {
    pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut prison_after_n_day = vec![];
        prison_after_n_day.push(cells.clone());
        println!("{:?}", prison_after_n_day);

            cells[0] = 0;
            cells[7] = 0;
        for _ in 1..=15 {
            let prev_cells = prison_after_n_day.last().unwrap();
            for i in 1..7 {
                if prev_cells[i+1] == prev_cells[i-1] {
                    cells[i] = 1;
                } else {
                    cells[i] = 0;
                }
            }
            println!("{:?}", cells);
            prison_after_n_day.push(cells.clone());
        }
        prison_after_n_day[(n as usize - 1) % 14 + 1].clone()
    }
}


fn main() {
    let mut cells = vec![0,1,0,1,1,0,0,1];
    let n = 7;
    let ans = Solution::prison_after_n_days(cells, n);
    print!("{:?}", ans);
}
