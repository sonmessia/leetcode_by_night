use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Eq, PartialEq, Hash, Clone)]
struct Packet {
    source: i32,
    destination: i32,
    timestamp: i32,
}

struct Router {
    packets: VecDeque<Packet>,
    map: HashMap<i32, VecDeque<i32>>,
    set: HashSet<Packet>,
    limit: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    fn new(memoryLimit: i32) -> Self {
        Self {
            packets: VecDeque::with_capacity(memoryLimit as usize),
            map: HashMap::new(),
            set: HashSet::new(),
            limit: memoryLimit as usize,
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = Packet {
            source,
            destination,
            timestamp,
        };
        let ans = if !self.set.insert(packet.clone()) {
            false
        } else {
            if self.packets.len() == self.limit {
                self.pop_packet();
            }

            self.packets.push_back(packet.clone());
            self.map
                .entry(destination)
                .or_insert_with(VecDeque::new)
                .push_back(timestamp);
            true
        };
        ans
    }

    fn pop_packet(&mut self) -> Option<Packet> {
        if let Some(packet) = self.packets.pop_front() {
            self.set.remove(&packet);
            self.map.entry(packet.destination).and_modify(|timestamps| {
                timestamps.pop_front();
            });
            Some(packet)
        } else {
            None
        }
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        let res = if let Some(packet) = self.pop_packet() {
            vec![packet.source, packet.destination, packet.timestamp]
        } else {
            vec![]
        };
        res
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(timestamps) = self.map.get(&destination) {
            let p1 = timestamps.partition_point(|&x| x < start_time);
            let p2 = timestamps.partition_point(|&x| x <= end_time);
            (p2 - p1) as i32
        } else {
            0
        }
    }
}
