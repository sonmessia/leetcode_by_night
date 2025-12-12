struct Solution;

impl Solution {
    pub fn count_mentions(number_of_users: i32, mut events: Vec<Vec<String>>) -> Vec<i32> {
        let n = number_of_users as usize;

        events.sort_by(|a, b| {
            let time_a = a[1].parse::<i32>().unwrap();
            let time_b = b[1].parse::<i32>().unwrap();
            time_a
                .cmp(&time_b)
                .then_with(|| (a[0] != "OFFLINE").cmp(&(b[0] != "OFFLINE")))
        });
        let mut ans = vec![0; n];

        let mut offline_users = vec![0; n];

        for e in events {
            let curr_time = e[1].parse::<i32>().unwrap();

            match e[0].as_str() {
                "MESSAGE" => match e[2].as_str() {
                    "ALL" => ans.iter_mut().for_each(|x| *x += 1),
                    "HERE" => {
                        for i in 0..n {
                            if offline_users[i] <= curr_time {
                                ans[i] += 1;
                            }
                        }
                    }
                    other => {
                        for id in other.split_whitespace() {
                            let idx: usize = id[2..].parse().unwrap();
                            ans[idx] += 1;
                        }
                    }
                },
                _ => {
                    let idx: usize = e[2].parse().unwrap();
                    offline_users[idx] = curr_time + 60;
                }
            }
        }

        ans
    }
}
