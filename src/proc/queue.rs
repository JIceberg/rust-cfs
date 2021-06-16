// a task priority queue that holds all born tasks in system

use super::task::{Task, TaskStatus};
use std::collections::HashMap;

pub struct TaskQueue {
    tasks: HashMap<u128, Vec<Task>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self { tasks: HashMap::new() }
    }

    pub fn add(&mut self, task: Task) {
        if task.get_status() == TaskStatus::Idle || task.get_status() == TaskStatus::Terminated {
            return;
        }
        let start_time = task.get_start_time();
        if !self.tasks.contains_key(&start_time) {
            self.tasks.insert(start_time, Vec::new());
        }
        self.tasks.get_mut(&start_time)
            .unwrap()
            .push(task);
    }

    pub fn append(&mut self, tasks: &[Task]) {
        for task in tasks {
            self.add(*task);
        }
    }

    pub fn pop(&mut self) -> Vec<Task> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut smallest_key = 0;
        for key in self.tasks.keys() {
            if smallest_key > *key || smallest_key == 0 {
                smallest_key = *key;
            }
        }
        
        self.tasks.remove(&smallest_key).unwrap()
    }

    pub fn remove(&mut self, time: u128) -> Vec<Task> {
        if self.is_empty() {
            return Vec::new();
        }

        self.tasks.remove(&time).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}
