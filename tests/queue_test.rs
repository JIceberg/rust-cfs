#![cfg(test)]

extern crate rust_cfs as cfs;

use cfs::proc::task::{Task, TaskStatus};
use cfs::proc::queue::TaskQueue;

#[test]
fn test_popping() {
    let task_one    = Task::new(1, 13191, 10, 5, 1);
    let task_two    = Task::new(2, 13289, 10, 5, 1);
    let task_three  = Task::new(3, 139, 10, 5, 2);
    let task_four   = Task::new(4, 31921, 5, 10, 3);
    let task_five   = Task::new(5, 3874, 7, 3, 5);
    let task_six    = Task::new(6, 17013, 10, 6, 5);

    let mut task_queue = TaskQueue::new();
    
    task_queue.add(task_one);
    task_queue.add(task_two);
    task_queue.add(task_three);
    task_queue.add(task_four);
    task_queue.add(task_five);
    task_queue.add(task_six);

    let mut idx = 1;
    while !task_queue.is_empty() {
        let tasks = task_queue.pop();
        for mut task in tasks {
            task.schedule();
            assert_eq!(task.get_start_time(), idx);
            assert_eq!(task.get_status(), TaskStatus::Waiting);
        }
        idx = match idx {
            1 => 2,
            2 => 3,
            3 => 5,
            _ => 0
        };
    }
    assert_eq!(idx, 0);
}

#[test]
fn test_remove() {
    let task_one    = Task::new(1, 13191, 10, 5, 1);
    let task_two    = Task::new(2, 13289, 10, 5, 1);
    let task_three  = Task::new(3, 139, 10, 5, 2);
    let task_four   = Task::new(4, 31921, 5, 10, 3);
    let task_five   = Task::new(5, 3874, 7, 3, 5);
    let task_six    = Task::new(6, 17013, 10, 6, 5);

    let mut task_queue = TaskQueue::new();
    
    task_queue.add(task_one);
    task_queue.add(task_two);
    task_queue.add(task_three);
    task_queue.add(task_four);
    task_queue.add(task_five);
    task_queue.add(task_six);

    let mut idx = 1;
    while !task_queue.is_empty() {
        let tasks = task_queue.remove(idx);
        for mut task in tasks {
            task.schedule();
            assert_eq!(task.get_start_time(), idx);
            assert_eq!(task.get_status(), TaskStatus::Waiting);
        }
        idx = match idx {
            1 => 2,
            2 => 3,
            3 => 5,
            _ => 0
        };
    }
    assert_eq!(idx, 0);
}
