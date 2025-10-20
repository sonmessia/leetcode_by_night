struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut value = 0;

        for operation in operations {
            match operation.as_str() {
                "++X" | "X++" => value += 1,
                "--X" | "X--" => value -= 1,
                _ => {}
            }
        }
        value
    }
}
