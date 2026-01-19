struct Solution;

impl Solution {
    fn cost(minute: i32, second: i32, start_at: i32, move_cost: i32, push_cost: i32) -> i32 {
        let s: String = (minute * 100 + second).to_string();

        let mut curr = (start_at as u8 + b'0') as char;
        let mut ans = 0;

        for c in s.chars() {
            if c == curr {
                ans += push_cost;
            } else {
                ans += move_cost + push_cost;
                curr = c;
            }
        }
        ans
    }
    pub fn min_cost_set_time(
        start_at: i32,
        move_cost: i32,
        push_cost: i32,
        target_seconds: i32,
    ) -> i32 {
        let minute = target_seconds / 60;
        let second = target_seconds % 60;

        return Self::cost(minute, second, start_at, move_cost, push_cost).min(Self::cost(
            minute - 1,
            second + 60,
            start_at,
            move_cost,
            push_cost,
        ));
    }
}
