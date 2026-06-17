struct Solution;

impl Solution {
    pub fn process_str(s: String, k: i64) -> char {
        let mut count = 0i64;
        let mut k_i = k;

        for c in s.chars() {
            match c {
                '*' => {
                    count -= 1;
                    count = count.max(0);
                }
                '#' => {
                    count *= 2;
                }
                '%' => (),
                _ => {
                    count += 1;
                }
            }
        }

        println!("count: {count}");
        if k >= count {
            return '.';
        }

        for c in s.chars().rev() {
            match c {
                '*' => {
                    count += 1;
                }
                '#' => {
                    count /= 2;
                    if k_i >= count {
                        k_i -= count;
                    }
                }
                '%' => {
                    k_i = count - k_i - 1;
                }
                _ => {
                    count -= 1;
                    if k_i >= count {
                        return c;
                    }
                }
            }
        }
        return '.';
    }
}
