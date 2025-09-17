struct Solution;

impl Solution {
    pub fn most_popular_creator(
        creators: Vec<String>,
        ids: Vec<String>,
        views: Vec<i32>,
    ) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut creator_views: HashMap<&String, i32> = HashMap::new();
        let mut creator_videos: HashMap<&String, (&String, i32)> = HashMap::new();
        for i in 0..creators.len() {
            let creator = &creators[i];
            let id = &ids[i];
            let view = views[i];
            *creator_views.entry(creator).or_insert(0) += view;
            let entry = creator_videos.entry(creator).or_insert((id, view));
            if view > entry.1 || (view == entry.1 && id < entry.0) {
                *entry = (id, view);
            }
        }
        let max_views = *creator_views.values().max().unwrap_or(&0);
        let mut result = Vec::new();
        for (creator, &total_views) in &creator_views {
            if total_views == max_views {
                if let Some(&(id, _)) = creator_videos.get(creator) {
                    result.push(vec![(*creator).clone(), id.clone()]);
                }
            }
        }
        result
    }
}

fn main() {
    let creators = vec!["alice".to_string(), "bob".to_string(), "alice".to_string()];
    let ids = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let views = vec![5, 10, 5];
    let result = Solution::most_popular_creator(creators, ids, views);
    for entry in result {
        println!("{:?}", entry);
    }
}
