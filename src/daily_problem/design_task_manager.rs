use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct TaskEntry {
    priority: i32,
    task_id: i32,
    user_id: i32,
}

impl Ord for TaskEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.priority.cmp(&other.priority).reverse() {
            Ordering::Equal => self.task_id.cmp(&other.task_id).reverse(),
            ord => ord,
        }
    }
}

impl PartialOrd for TaskEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct TaskManager {
    task: HashMap<i32, (i32, i32)>,
    all_tasks: BTreeSet<TaskEntry>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task = HashMap::new();
        let mut all_tasks = BTreeSet::new();
        for t in tasks {
            let user_id = t[0];
            let task_id = t[1];
            let priority = t[2];
            task.insert(task_id, (user_id, priority));
            all_tasks.insert(TaskEntry {
                priority,
                task_id,
                user_id,
            });
        }
        TaskManager { task, all_tasks }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.task.insert(task_id, (user_id, priority));
        self.all_tasks.insert(TaskEntry {
            priority,
            task_id,
            user_id,
        });
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some(&(user_id, old_priority)) = self.task.get(&task_id) {
            let old_entry = TaskEntry {
                priority: old_priority,
                task_id,
                user_id,
            };
            self.all_tasks.remove(&old_entry);
            let new_entry = TaskEntry {
                priority: new_priority,
                task_id,
                user_id,
            };
            self.all_tasks.insert(new_entry);
            self.task.insert(task_id, (user_id, new_priority));
        }
    }

    fn rmv(&mut self, task_id: i32) {
        if let Some(&(user_id, priority)) = self.task.get(&task_id) {
            self.all_tasks.remove(&TaskEntry {
                priority,
                task_id,
                user_id,
            });
            self.task.remove(&task_id);
        }
    }

    fn exec_top(&mut self) -> i32 {
        if let Some(top_task) = self.all_tasks.iter().next().cloned() {
            self.all_tasks.remove(&top_task);
            self.task.remove(&top_task.task_id);
            return top_task.user_id;
        }
        -1
    }
}

fn main() {
    let mut task_manager =
        TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
    println!("{:#?}", task_manager.all_tasks);
    println!("------------------------------------");
    task_manager.add(4, 104, 5);
    println!("{:#?}", task_manager.all_tasks);
    println!("------------------------------------");
    task_manager.edit(102, 8);
    println!("{:#?}", task_manager.all_tasks);
    println!("------------------------------------");
    println!("Top priority {:?}", task_manager.exec_top()); // task_id có priority lớn nhất
    task_manager.rmv(101);
    println!("{:#?}", task_manager.all_tasks);
    println!("------------------------------------");
    task_manager.add(5, 105, 15);
    println!("{:#?}", task_manager.all_tasks);
    println!("------------------------------------");
    println!("Top priority {:?}", task_manager.exec_top());
}
