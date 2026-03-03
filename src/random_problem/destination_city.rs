struct Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut map = std::collections::HashMap::new();
        for path in &paths {
            map.insert(path[0].clone(), path[1].clone());
        }
        let mut city = paths[0][0].clone();
        while let Some(next_city) = map.get(&city) {
            city = next_city.clone();
        }
        city
    }
}
