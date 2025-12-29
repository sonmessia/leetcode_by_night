struct Solution;

impl Solution {
    pub fn pyramic_transition(bottom: String, allowed: Vec<String>) -> bool {
        use std::collections::HashMap;
        let mut map: HashMap<(u8, u8), Vec<u8>> = HashMap::new();
        for s in allowed {
            let bytes = s.as_bytes();
            map.entry((bytes[0], bytes[1])).or_default().push(bytes[2]);
        }

        fn dfs(
            curr: &[u8],
            next: &mut Vec<u8>,
            map: &HashMap<(u8, u8), Vec<u8>>,
            index: usize,
        ) -> bool {
            if index == curr.len() - 1 {
                if curr.len() == 1 {
                    return true;
                }
                return dfs(next, &mut Vec::new(), map, 0);
            }

            if let Some(candidates) = map.get(&(curr[index], curr[index + 1])) {
                for &c in candidates {
                    next.push(c);
                    if dfs(curr, next, map, index + 1) {
                        return true;
                    }
                    next.pop();
                }
            }
            false
        }

        dfs(bottom.as_bytes(), &mut Vec::new(), &map, 0)
    }
}
