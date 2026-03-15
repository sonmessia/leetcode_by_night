const MOD: i64 = 1_000_000_007;

struct Fancy {
    seq: Vec<i64>,
    add: i64,
    mult: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {
    fn new() -> Self {
        Self {
            seq: Vec::new(),
            add: 0,
            mult: 1,
        }
    }

    fn pow(mut base: i64, mut exp: i64) -> i64 {
        let mut res = 1;
        base %= MOD;

        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % MOD;
            }
            base = (base * base) % MOD;
            exp /= 2;
        }
        res
    }

    fn append(&mut self, val: i32) {
        let inv_mult = Self::pow(self.mult, MOD - 2);

        let mut diff = (val as i64 - self.add + MOD) % MOD;

        if diff < 0 {
            diff += MOD;
        }

        let base_val = (diff * inv_mult) % MOD;
        self.seq.push(base_val);
    }

    fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64) % MOD;
    }

    fn mult_all(&mut self, m: i32) {
        let m_i64 = m as i64;
        self.mult = (self.mult * m_i64) % MOD;
        self.add = (self.add * m_i64) % MOD;
    }

    fn get_index(&self, idx: i32) -> i32 {
        let i = idx as usize;
        if i >= self.seq.len() {
            return -1;
        }

        (((self.seq[i] * self.mult) as i64 % MOD + self.add as i64) % MOD) as i32
    }
}

/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */
