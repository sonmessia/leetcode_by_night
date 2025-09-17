use std::collections::{BTreeSet, HashMap};

#[derive(Eq, PartialEq)]
struct FoodEntry {
    rating: i32,
    name: String,
}

impl Ord for FoodEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .rating
            .cmp(&self.rating)
            .then(self.name.cmp(&other.name))
    }
}

impl PartialOrd for FoodEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct FoodRatings {
    // food -> (cuisine, rating)
    food_info: HashMap<String, (String, i32)>,
    // cuisine -> set of FoodEntry
    cuisine_foods: HashMap<String, BTreeSet<FoodEntry>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_info = HashMap::new();
        let mut cuisine_foods: HashMap<String, BTreeSet<FoodEntry>> = HashMap::new();
        for i in 0..foods.len() {
            let food = foods[i].clone();
            let cuisine = cuisines[i].clone();
            let rating = ratings[i];
            food_info.insert(food.clone(), (cuisine.clone(), rating));
            cuisine_foods
                .entry(cuisine)
                .or_insert_with(BTreeSet::new)
                .insert(FoodEntry { rating, name: food });
        }
        Self {
            food_info,
            cuisine_foods,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        if let Some((cuisine, old_rating)) = self.food_info.get_mut(&food) {
            // xóa entry cũ khỏi BTreeSet
            if let Some(foods_set) = self.cuisine_foods.get_mut(cuisine) {
                foods_set.remove(&FoodEntry {
                    rating: *old_rating,
                    name: food.clone(),
                });
                foods_set.insert(FoodEntry {
                    rating: new_rating,
                    name: food.clone(),
                });
            }
            // cập nhật rating
            *old_rating = new_rating;
        }
    }

    fn highest_rated(&self, cuisine: String) -> String {
        self.cuisine_foods
            .get(&cuisine)
            .and_then(|set| set.iter().next().map(|entry| entry.name.clone()))
            .unwrap_or_default()
    }
}

fn main() {
    let foods = vec![
        "kimchi".to_string(),
        "miso".to_string(),
        "sushi".to_string(),
        "ramen".to_string(),
        "bulgogi".to_string(),
    ];
    let cuisines = vec![
        "korean".to_string(),
        "japanese".to_string(),
        "japanese".to_string(),
        "japanese".to_string(),
        "korean".to_string(),
    ];
    let ratings = vec![9, 12, 8, 15, 7];
    let mut food_ratings = FoodRatings::new(foods, cuisines, ratings);

    println!("Design a Food Rating System");
    println!("---------------------------");
    println!("{:?}", food_ratings.food_info);

    println!("{:?}", food_ratings.highest_rated("korean".to_string())); // kimchi
    println!("{:?}", food_ratings.highest_rated("japanese".to_string())); // ramen
    food_ratings.change_rating("sushi".to_string(), 16);
    println!("{:?}", food_ratings.highest_rated("japanese".to_string())); // sushi
    food_ratings.change_rating("ramen".to_string(), 16);
    println!("{:?}", food_ratings.highest_rated("japanese".to_string())); // ramen
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_food_ratings() {
        let foods = vec![
            "kimchi".to_string(),
            "miso".to_string(),
            "sushi".to_string(),
            "ramen".to_string(),
            "bulgogi".to_string(),
        ];
        let cuisines = vec![
            "korean".to_string(),
            "japanese".to_string(),
            "japanese".to_string(),
            "japanese".to_string(),
            "korean".to_string(),
        ];
        let ratings = vec![9, 12, 8, 15, 7];
        let mut food_ratings = FoodRatings::new(foods, cuisines, ratings);
        assert_eq!(
            food_ratings.highest_rated("korean".to_string()),
            "kimchi".to_string()
        );
        assert_eq!(
            food_ratings.highest_rated("japanese".to_string()),
            "ramen".to_string()
        );
        food_ratings.change_rating("sushi".to_string(), 16);
        assert_eq!(
            food_ratings.highest_rated("japanese".to_string()),
            "sushi".to_string()
        );
        food_ratings.change_rating("ramen".to_string(), 16);
        assert_eq!(
            food_ratings.highest_rated("japanese".to_string()),
            "ramen".to_string()
        );
    }
}
