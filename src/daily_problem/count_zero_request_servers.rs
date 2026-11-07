use std::collections::{BTreeSet, HashMap};

struct Solution;

impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let mut logs = logs;
        logs.sort_by(|a, b| a[1].cmp(&b[1]));

        let m = queries.len();
        let mut d = vec![];

        for i in 0..m {
            d.push((queries[i], i));
        }

        d.sort();

        let mut s = BTreeSet::<(i32, i32)>::new();
        let mut mp = HashMap::<i32, i32>::new();
        let mut ans = vec![0; m];

        let mut j = 0;

        for i in 0..m {
            while j < logs.len() && logs[j][1] <= d[i].0 {
                s.insert((logs[j][1], logs[j][0]));
                *mp.entry(logs[j][0]).or_insert(0) += 1;
                j += 1;
            }

            while let Some(&(time, server)) = s.iter().next() {
                if time < d[i].0 - x {
                    s.remove(&(time, server));
                    *mp.entry(server).or_insert(0) -= 1;
                    if mp[&server] == 0 {
                        mp.remove(&server);
                    }
                } else {
                    break;
                }
            }

            ans[d[i].1] = n - mp.len() as i32;
        }

        ans
    }
}
