use std::collections::HashMap;

struct Allocator {
    mem: Vec<i32>,
    n: usize,
    mp: HashMap<i32, Vec<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Allocator {
    fn new(n: i32) -> Self {
        /*  */
        let n = n as usize;
        Self {
            mem: vec![0; n],
            n,
            mp: HashMap::new(),
        }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut free = 0;
        let mut idx = 0;

        for i in 0..self.n {
            if free == 0 {
                idx = i;
            }
            if self.mem[i] == 0 {
                free += 1;
            } else {
                free = 0;
            }

            if free >= size {
                break;
            }
        }

        if free >= size {
            let v = self.mp.entry(m_id).or_insert(vec![]);
            for i in idx..idx + size as usize {
                self.mem[i] = m_id;
                v.push(i);
            }
            idx as i32
        } else {
            -1
        }
    }

    fn free_memory(&mut self, m_id: i32) -> i32 {
        let mut cnt = 0;

        for &idx in self.mp.get(&m_id).unwrap_or(&vec![]) {
            self.mem[idx] = 0;
            cnt += 1;
        }

        self.mp.remove(&m_id);
        cnt
    }
}

/**
 * Your Allocator object will be instantiated and called as such:
 * let obj = Allocator::new(n);
 * let ret_1: i32 = obj.allocate(size, mID);
 * let ret_2: i32 = obj.free_memory(mID);
 */

#[test]
fn name() {
    todo!();
}
