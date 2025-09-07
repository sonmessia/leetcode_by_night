struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, f64)> = position
            .into_iter()
            .zip(speed.into_iter())
            .map(|(p, s)| (p, (target - p) as f64 / s as f64))
            .collect();
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let mut fleets = 0;
        let mut last_time = 0.0;
        for &(_, time) in &cars {
            if time > last_time {
                fleets += 1;
                last_time = time;
            }
        }
        fleets
    }
}
