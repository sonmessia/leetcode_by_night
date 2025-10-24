struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let n = s.len();
        let mut arr = s.bytes().collect::<Vec<u8>>();

        for i in 1..n - 2 {
            for j in 0..=n - i - 1 {
                let a = arr[j] - b'0';
                let b = arr[j + i] - b'0';
                arr[j] = ((a + b) % 10) + b'0';
            }
        }

        arr[0] == arr[1]
    }
}
