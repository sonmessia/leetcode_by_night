struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let count = start_time
            .iter()
            .zip(end_time.iter())
            .filter(|&(&start, &end)| start <= query_time && query_time <= end)
            .count();
        count as i32
    }
}
