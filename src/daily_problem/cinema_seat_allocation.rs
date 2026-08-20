use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut reserved_map: HashMap<i32, u8> = std::collections::HashMap::new();

        for seat in reserved_seats {
            let row = seat[0];
            let col = seat[1];
            if col >= 2 && col <= 9 {
                *reserved_map.entry(row).or_insert(0) |= 1 << (col - 2);
            }
        }

        // for (row, mask) in reserved_map.iter() {
        //     println!("Row: {}, Mask: {:09b}", row, mask);
        // }

        let mut ans = (n - reserved_map.len() as i32) * 2;

        for (_, mask) in reserved_map.iter() {
            let (left, middle, right) = (
                mask & 0b00001111 == 0,
                mask & 0b11110000 == 0,
                mask & 0b00111100 == 0,
            );

            if left && right {
                ans += 2;
            } else if left || middle || right {
                ans += 1;
            }
        }

        ans
    }
}
