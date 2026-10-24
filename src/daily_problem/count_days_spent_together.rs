struct Solution;

impl Solution {
    pub fn count_days_spent_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let parse_date = |date_str: &str| {
            let parts: Vec<&str> = date_str.split('-').collect();
            let month = parts[0].parse::<u32>().unwrap();
            let day = parts[1].parse::<u32>().unwrap();

            let mut days_in_year = 0;
            let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

            for m in 1..month {
                days_in_year += days_in_month[m as usize];
            }
            days_in_year += day;

            days_in_year
        };

        let arrive_alice = parse_date(&arrive_alice);
        let leave_alice = parse_date(&leave_alice);
        let arrive_bob = parse_date(&arrive_bob);
        let leave_bob = parse_date(&leave_bob);

        let start = std::cmp::max(arrive_alice, arrive_bob);
        let end = std::cmp::min(leave_alice, leave_bob);

        if start > end {
            0
        } else {
            (end - start) as i32 + 1
        }
    }
}
