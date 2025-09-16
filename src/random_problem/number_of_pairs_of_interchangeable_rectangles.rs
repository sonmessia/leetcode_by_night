struct Solution;

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        fn combination(n: i64) -> i64 {
            n * (n - 1) / 2
        }
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }
        use std::collections::HashMap;
        let mut map: HashMap<(i32, i32), i64> = HashMap::new();

        for rect in rectangles {
            let (w, h) = (rect[0], rect[1]);
            let g = gcd(w, h);
            let ratio = (w / g, h / g);
            *map.entry(ratio).or_insert(0) += 1;
        }

        let mut result = 0;
        for &count in map.values() {
            if count > 1 {
                result += combination(count);
            }
        }
        result
    }
}
