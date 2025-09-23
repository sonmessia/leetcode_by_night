struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<&str> = version1.split('.').collect();
        let v2: Vec<&str> = version2.split('.').collect();
        let len = v1.len().max(v2.len());

        for i in 0..len {
            let num1 = if i < v1.len() {
                v1[i].parse::<i32>().unwrap()
            } else {
                0
            };
            let num2 = if i < v2.len() {
                v2[i].parse::<i32>().unwrap()
            } else {
                0
            };

            // println!("Comparing: {} and {}", num1, num2);
            match (num1, num2) {
                (0, 0) => continue,
                (n1, n2) if n1 > n2 => return 1,
                (n1, n2) if n2 > n1 => return -1,
                _ => {}
            }
        }
        0
    }
}

fn main() {
    let version1 = "1.2".to_string();
    let version2 = "1.10".to_string();
    let result = Solution::compare_version(version1, version2);
    println!("Result: {}", result); // Output: Result: 0
}
