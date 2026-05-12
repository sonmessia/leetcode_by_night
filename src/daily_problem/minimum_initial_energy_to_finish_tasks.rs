struct Solution;

impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by_key(|task| task[1] - task[0]);
        tasks.iter().fold(0, |effort, task| {
            task[1].max(effort + task[0])
        })
    }
}
