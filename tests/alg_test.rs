#![cfg(test)]

extern crate rust_cfs as cfs;

use cfs::proc::task::{Task, TaskStatus};
use cfs::proc::queue::TaskQueue;
use cfs::sched::{clock::Clock, fair::FairAlgorithm};

#[test]
fn test_no_update() {
    let task_one    = Task::new(1, 13191, 10, 5, 1, 5);
    let task_two    = Task::new(2, 13289, 10, 5, 1, 4);
    let task_three  = Task::new(3, 139, 10, 5, 2, 8);
    let task_four   = Task::new(4, 31921, 5, 10, 3, 4);
    let task_five   = Task::new(5, 3874, 7, 3, 5, 2);
    let task_six    = Task::new(6, 17013, 10, 6, 5, 5);

    let mut task_queue = TaskQueue::new();

    task_queue.append(&[
        task_one,
        task_two,
        task_three,
        task_four,
        task_five,
        task_six
    ]);

    let mut sysclock = Clock::new();
    let mut rq = FairAlgorithm::new(&mut sysclock);

    sysclock.tick();

    while !task_queue.is_empty() {
        let tasks = task_queue.pop();
        rq.push(tasks);
        sysclock.tick();
    }

    let first = rq.pop();
    assert_eq!(first.get_id(), 1);

    sysclock.tick();

    let second = rq.pop();
    assert_eq!(second.get_id(), 2);

    sysclock.tick();

    let third = rq.pop();
    assert_eq!(third.get_id(), 3);
}

#[test]
fn test_with_update() {
    let task_one    = Task::new(1, 13191, 10, 5, 1, 5);
    let task_two    = Task::new(2, 13289, 10, 5, 1, 4);
    let task_three  = Task::new(3, 139, 10, 5, 2, 8);
    let task_four   = Task::new(4, 31921, 5, 10, 3, 4);
    let task_five   = Task::new(5, 3874, 7, 3, 5, 2);
    let task_six    = Task::new(6, 17013, 10, 6, 5, 5);

    let mut task_queue = TaskQueue::new();

    task_queue.append(&[
        task_one,
        task_two,
        task_three,
        task_four,
        task_five,
        task_six
    ]);

    let mut sysclock = Clock::new();
    let mut rq = FairAlgorithm::new(&mut sysclock);

    sysclock.tick();

    while !task_queue.is_empty() {
        let tasks = task_queue.pop();
        rq.push(tasks);
        sysclock.tick();
    }

    let mut curr: Task = rq.pop();
    assert_eq!(curr.get_id(), 1);

    curr.cpu_cycle();
    sysclock.tick();
    rq.insert(curr);

    curr = rq.pop();
    assert_eq!(curr.get_id(), 2);

    curr.cpu_cycle();
    sysclock.tick();
    rq.insert(curr);

    curr = rq.pop();
    assert_eq!(curr.get_id(), 3);
}
