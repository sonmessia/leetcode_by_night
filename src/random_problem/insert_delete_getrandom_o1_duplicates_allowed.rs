use rand::Rng;
use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    data: Vec<i32>,
    map: HashMap<i32, HashSet<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn has_val(&self, val: i32) -> bool {
        self.map.get(&val).map_or(false, |set| !set.is_empty())
    }

    fn insert(&mut self, val: i32) -> bool {
        let exists = self.has_val(val);

        self.map
            .entry(val)
            .or_insert_with(HashSet::new)
            .insert(self.data.len());

        self.data.push(val);

        !exists
    }

    fn remove(&mut self, val: i32) -> bool {
        let exists = self.has_val(val);
        if exists {
            let set = self.map.get_mut(&val).unwrap();
            let i = *set.iter().next().unwrap();
            set.remove(&i);

            self.data.swap_remove(i);
            if i < self.data.len() {
                let v = self.data[i];
                let v_set = self.map.get_mut(&v).unwrap();
                v_set.remove(&self.data.len());
                v_set.insert(i);
            }
        }
        exists
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::rng();
        let i = rng.random_range(0..self.data.len());
        self.data[i]
    }
}

// /**
//  * Your RandomizedCollection object will be instantiated and called as such:
//  * let obj = RandomizedCollection::new();
//  * let ret_1: bool = obj.insert(val);
//  * let ret_2: bool = obj.remove(val);
//  * let ret_3: i32 = obj.get_random();
//  */
