struct Solution;

impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        let mut mass = mass as i64;
        asteroids.sort_unstable();
        for asteroid in asteroids {
            if mass < asteroid as i64 {
                return false;
            }
            mass += asteroid as i64;
        }
        true
    }
}
