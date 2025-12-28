use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut count = vec![0; n];

        // Sắp xếp các cuộc họp theo thời gian bắt đầu
        let mut meetings = meetings;
        meetings.sort_unstable_by_key(|m| m[0]);

        // unused_rooms: Min-Heap lưu index của phòng
        let mut unused_rooms = BinaryHeap::new();
        for i in 0..n {
            unused_rooms.push(Reverse(i));
        }

        // used_rooms: Min-Heap lưu (thời_gian_kết_thúc, index_phòng)
        // Dùng i64 cho thời gian để tránh tràn số
        let mut used_rooms: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        for meeting in meetings {
            let start = meeting[0] as i64;
            let end = meeting[1] as i64;

            // 1. Giải phóng các phòng đã dùng xong trước khi cuộc họp mới bắt đầu
            // Kiểm tra phần tử đầu heap (nhỏ nhất)
            while let Some(&Reverse((finished_time, room_idx))) = used_rooms.peek() {
                if finished_time <= start {
                    used_rooms.pop();
                    unused_rooms.push(Reverse(room_idx));
                } else {
                    break;
                }
            }

            // 2. Phân phối phòng
            let room_idx;
            if let Some(Reverse(r)) = unused_rooms.pop() {
                // Có phòng trống
                room_idx = r;
                used_rooms.push(Reverse((end, room_idx)));
            } else {
                // Không có phòng trống, phải đợi phòng sớm nhất
                let Reverse((avail_time, r)) = used_rooms.pop().unwrap();
                room_idx = r;
                let duration = end - start;
                used_rooms.push(Reverse((avail_time + duration, room_idx)));
            }

            // Tăng biến đếm
            count[room_idx] += 1;
        }

        // Tìm phòng có số lượt dùng nhiều nhất (và index nhỏ nhất nếu bằng nhau)
        let max = *count.iter().max().unwrap();
        count.iter().position(|&c| c == max).unwrap() as i32
    }
}
