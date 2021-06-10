#![cfg(test)]

extern crate rust_cfs as cfs;
extern crate raw_pointer as rptr;

use cfs::proc::task::{Task, TaskStatus};
use rptr::Pointer;

#[test]
fn test_execute() {
    let mut x = 3;

    let mut tasks: Vec<Pointer<Task>> = Vec::new();

    let mut my_task = Task::new(1, 5, 0, 1);

    my_task.schedule();

    tasks.push(Pointer::new(&mut my_task));

    while tasks.len() > 0 {
        let y = tasks.len();
        for i in 0..y {
            let mut task = tasks[i];
            println!("Performing sequence for task {:?}", task.get_id());
            match task.get_status() {
                TaskStatus::Running => {
                    match task.get_id() {
                        1 => {
                            x = x + 1;
                        },
                        _ => println!("Executing task {:?}", task.get_id())
                    };
                },
                TaskStatus::Waiting => {
                    task.run();
                },
                _ => {
                    tasks.remove(i);
                }
            };
            task.cpu_cycle();
            task.io_cycle();
        }
    }

    assert_eq!(x, 8);
}
