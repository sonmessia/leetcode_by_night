use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap, HashSet};

struct MovieRentingSystem {
    // (shop, movie), price
    entries: HashMap<(i32, i32), i32>,
    // movie -> (price, shop)
    available: HashMap<i32, BTreeSet<(i32, i32)>>,
    // (price, shop, movie)
    rented: BTreeSet<(i32, i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut system = MovieRentingSystem {
            entries: HashMap::new(),
            available: HashMap::new(),
            rented: BTreeSet::new(),
        };
        for entry in entries {
            let shop = entry[0];
            let movie = entry[1];
            let price = entry[2];
            system.entries.insert((shop, movie), price);
            system
                .available
                .entry(movie)
                .or_insert_with(BTreeSet::new)
                .insert((price, shop));
        }
        system
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        if let Some(shops) = self.available.get(&movie) {
            shops.iter().take(5).map(|&(_, shop)| shop).collect()
        } else {
            vec![]
        }
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        if let Some(&price) = self.entries.get(&(shop, movie)) {
            if let Some(shops) = self.available.get_mut(&movie) {
                shops.remove(&(price, shop));
                if shops.is_empty() {
                    self.available.remove(&movie);
                }
            }
            self.rented.insert((price, shop, movie));
        }
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        if let Some(&price) = self.entries.get(&(shop, movie)) {
            if self.rented.remove(&(price, shop, movie)) {
                self.available
                    .entry(movie)
                    .or_insert_with(BTreeSet::new)
                    .insert((price, shop));
            }
        }
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rented
            .iter()
            .take(5)
            .map(|&(price, shop, movie)| vec![shop, movie])
            .collect()
    }
}

// /**
//  * Your MovieRentingSystem object will be instantiated and called as such:
//  * let obj = MovieRentingSystem::new(n, entries);
//  * let ret_1: Vec<i32> = obj.search(movie);
//  * obj.rent(shop, movie);
//  * obj.drop(shop, movie);
//  * let ret_4: Vec<Vec<i32>> = obj.report();
//  */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_movie_rental_system() {
        let mut movie_renting_system = MovieRentingSystem::new(
            3,
            vec![
                vec![0, 1, 5],
                vec![0, 2, 6],
                vec![0, 3, 7],
                vec![1, 1, 4],
                vec![1, 2, 7],
            ],
        );
        assert_eq!(movie_renting_system.search(1), vec![1, 0]);
        movie_renting_system.rent(0, 1);
        assert_eq!(movie_renting_system.report(), vec![vec![0, 1]]);
        movie_renting_system.drop(0, 1);
        assert_eq!(movie_renting_system.report(), Vec::<Vec<i32>>::new());
        assert_eq!(movie_renting_system.search(2), vec![0, 1]);
        movie_renting_system.rent(0, 2);
        movie_renting_system.rent(1, 2);
        assert_eq!(movie_renting_system.report(), vec![vec![0, 2], vec![1, 2]]);
    }
}
