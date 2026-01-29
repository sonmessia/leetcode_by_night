struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut graph = vec![vec![0; 26]; 26];

        for i in 0..26 {
            graph[i][i] = 0;
            graph[i].iter_mut().for_each(|x| *x = i32::MAX / 2);
        }

        for i in 0..original.len() {
            let u = (original[i] as u8 - b'a') as usize;
            let v = (changed[i] as u8 - b'a') as usize;
            graph[u][v] = graph[u][v].min(cost[i]);
        }

        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    graph[i][j] = graph[i][j].min(graph[i][k] + graph[k][j]);
                }
            }
        }

        let mut ans: i64 = 0;

        for i in 0..source.len() {
            let u = source.as_bytes()[i] - b'a';
            let v = target.as_bytes()[i] - b'a';
            if u != v {
                if graph[u as usize][v as usize] >= i32::MAX / 2 {
                    return -1;
                } else {
                    ans += graph[u as usize][v as usize] as i64;
                }
            }
        }
        ans
    }
}
