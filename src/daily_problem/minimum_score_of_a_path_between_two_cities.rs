struct Solution;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashMap, HashSet, VecDeque};

        let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for road in roads {
            let u = road[0];
            let v = road[1];
            let score = road[2];
            graph.entry(u).or_insert(Vec::new()).push((v, score));
            graph.entry(v).or_insert(Vec::new()).push((u, score));
        }

        let mut visited: HashSet<i32> = HashSet::new();
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(1);
        visited.insert(1);

        let mut min_score = i32::MAX;

        while let Some(city) = queue.pop_front() {
            if let Some(neighbors) = graph.get(&city) {
                for &(neighbor, score) in neighbors {
                    min_score = min_score.min(score);
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        min_score
    }
}
