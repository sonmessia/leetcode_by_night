struct Solution;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut map = std::collections::HashMap::new();
        for piece in pieces {
            map.insert(piece[0], piece);
        }
        let mut i = 0;
        while i < arr.len() {
            if let Some(piece) = map.get(&arr[i]) {
                for &num in piece {
                    if arr[i] != num {
                        return false;
                    }
                    i += 1;
                }
            } else {
                return false;
            }
        }
        true
    }
}
