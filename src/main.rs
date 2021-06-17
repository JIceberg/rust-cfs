extern crate rust_cfs as cfs;

use cfs::sched::scheduler::Scheduler;
use cfs::proc::task::TaskChar;

fn main() {
    let task_one = TaskChar::new(1, 50, 9, 1, 4);
    let task_two = TaskChar::new(2, 50, 3, 7, 2);

    let tasks: Vec<TaskChar> = vec![task_one, task_two];

    let mut scheduler = Scheduler::new();

    scheduler.run(tasks);
}
