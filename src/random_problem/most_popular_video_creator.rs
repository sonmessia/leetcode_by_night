use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
struct CreatorInfo {
    total_views: i64,
    best_video_id: String,
    best_video_views: i64,
}

struct Solution;

impl Solution {
    pub fn most_popular_creator(
        creators: Vec<String>,
        ids: Vec<String>,
        views: Vec<i32>,
    ) -> Vec<Vec<String>> {
        let n = creators.len();
        let mut creator_map: HashMap<String, CreatorInfo> = HashMap::new();

        for i in 0..n {
            let creator = &creators[i];
            let video_id = &ids[i];
            let view_count = views[i] as i64;
            let entry = creator_map.entry(creator.clone()).or_insert(CreatorInfo {
                total_views: 0,
                best_video_id: video_id.clone(),
                best_video_views: 0,
            });
            entry.total_views += view_count;
            match view_count.cmp(&entry.best_video_views) {
                Ordering::Greater => {
                    entry.best_video_views = view_count;
                    entry.best_video_id = video_id.clone();
                }
                Ordering::Equal => {
                    if video_id < &entry.best_video_id {
                        entry.best_video_id = video_id.clone();
                    }
                }
                _ => {}
            }
        }

        println!("Creator Map: {:?}", creator_map);
        let mut ans = Vec::new();
        let max_views = creator_map
            .values()
            .map(|info| info.total_views)
            .max()
            .unwrap_or(0);
        println!("Max Views: {}", max_views);
        for (creator, info) in creator_map {
            if info.total_views == max_views {
                ans.push(vec![creator, info.best_video_id]);
            }
        }
        ans
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

    println!("-------------------------------------------");
    let creators = vec![
        "alice".to_string(),
        "alice".to_string(),
        "alice".to_string(),
    ];
    let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let views = vec![1, 2, 2];
    let result = Solution::most_popular_creator(creators, ids, views);

    for entry in result {
        println!("{:?}", entry);
    }
}
