extern crate rbtree;
extern crate raw_pointer as rptr;

use rbtree::RBTree;
use rptr::Pointer;
use super::clock::Clock;
use crate::proc::task::{Task, TaskStatus};
use std::collections::VecDeque;

pub struct FairAlgorithm {
    tree: RBTree<u64, Task>,
    idle: VecDeque<Task>,
    clock: Pointer<Clock>,
}

impl FairAlgorithm {
    pub fn new(clock: &mut Clock) -> Self {
        Self {
            tree: RBTree::new(),
            idle: VecDeque::new(),
            clock: Pointer::new(clock)
        }
    }

    #[inline]
    pub fn push(&mut self, tasks: Vec<Task>) {
        for task in tasks {
            self.insert(task);
        }
    }

    #[inline]
    pub fn insert(&mut self, mut task: Task) {
        let state = task.get_status();
        if state == TaskStatus::Terminated {
            return;
        } else if state == TaskStatus::Idle {
            self.idle.push_back(task);
            return;
        }
        let key: u64 = task.vruntime(self.clock.time());
        task.schedule();
        self.tree.insert(key, task);
    }

    #[inline]
    pub fn pop(&mut self) -> Task {
        let mut task = self.tree
                           .pop_first()
                           .unwrap()
                           .1;
        task.run();

        task
    }

    pub fn idle(&mut self) {
        let mut curr = self.idle.pop_front().unwrap();
        curr.io_cycle();
        self.insert(curr);
    }
}
