use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Videos {
    id: String,
    views: i32,
}

impl Ord for Videos {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.views.cmp(&other.views).reverse() {
            Ordering::Equal => self.id.cmp(&other.id).reverse(),
            ord => ord,
        }
    }
}

impl PartialOrd for Videos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;
impl Solution {
    pub fn most_popular_creator(
        creators: Vec<String>,
        ids: Vec<String>,
        views: Vec<i32>,
    ) -> Vec<Vec<String>> {
    }
}

fn main() {
    let creators = vec![
        "alice".to_string(),
        "bob".to_string(),
        "alice".to_string(),
        "christ".to_string(),
    ];
    let ids = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
    ];
    let views = vec![5, 10, 5, 4];
    let result = Solution::most_popular_creator(creators, ids, views);
    for entry in result {
        println!("{:?}", entry);
    }
}
